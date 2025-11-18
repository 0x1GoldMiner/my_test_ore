mod jito;
mod monitor;
mod price;
mod utils;

use clap::{Parser, Subcommand};
use monitor::{Monitor, MonitorSnapshot};
use ore_api::prelude::*;
use price::get_price_with_retry;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction,
    message::{v0, VersionedMessage},
    signature::{read_keypair_file, Keypair, Signer},
    transaction::VersionedTransaction,
};
use spl_token::amount_to_ui_amount;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};
use utils::*;

const DEFAULT_UNITS: u64 = 400_000;

#[derive(Parser)]
#[command(name = "ORE Test2 Optimized")]
#[command(about = "基于 ore_refined 设计思路优化的 ORE 挖矿程序", long_about = None)]
struct Cli {
    /// RPC 地址
    #[arg(long)]
    rpc: String,

    /// Keypair 文件路径
    #[arg(long)]
    keypair: String,

    /// 子命令
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 自动挖矿（阈值算法）
    AutoThreshold {
        /// 每个格子部署的 SOL 数量
        #[arg(long, default_value = "0.01")]
        amount_sol: f64,

        /// 阈值（SOL）
        #[arg(long, default_value = "0.01")]
        threshold_sol: f64,

        /// 最少满足条件的格子数量
        #[arg(long, default_value = "12")]
        min_squares: usize,

        /// 选择的格子数量
        #[arg(long, default_value = "5")]
        pick_squares: usize,

        /// 提前部署时间（秒）
        #[arg(long, default_value = "40.0")]
        start_before_seconds: f64,

        /// 剩余 slots 阈值（更精确的时机控制）
        #[arg(long, default_value = "15")]
        remaining_slots: u64,

        /// 最大部署 SOL 总量
        #[arg(long, default_value = "0.5")]
        max_total_amount_sol: f64,
    },

    /// 自动挖矿（最优化算法）
    AutoOptimized {
        /// 每个格子部署的 SOL 数量
        #[arg(long, default_value = "0.01")]
        amount_sol: f64,

        /// 最少满足条件的格子数量
        #[arg(long, default_value = "12")]
        min_squares: usize,

        /// 选择的格子数量
        #[arg(long, default_value = "5")]
        pick_squares: usize,

        /// 提前部署时间（秒）
        #[arg(long, default_value = "40.0")]
        start_before_seconds: f64,

        /// 剩余 slots 阈值
        #[arg(long, default_value = "15")]
        remaining_slots: u64,
    },

    /// 查看余额
    Balance,

    /// 领取奖励
    Claim,

    /// 查看当前状态
    Status,

    /// 查看 Board
    Board,

    /// 查看 Miner
    Miner,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // 读取 keypair
    let payer = Arc::new(
        read_keypair_file(&cli.keypair)
            .map_err(|e| anyhow::anyhow!("Failed to read keypair file: {}", e))?
    );
    info!("钱包地址: {}", payer.pubkey());

    // 创建 RPC 客户端（使用 processed 获得最快响应）
    let rpc = Arc::new(RpcClient::new_with_commitment(
        cli.rpc.clone(),
        CommitmentConfig::processed(),
    ));

    // 执行命令
    match cli.command {
        Commands::AutoThreshold {
            amount_sol,
            threshold_sol,
            min_squares,
            pick_squares,
            start_before_seconds,
            remaining_slots,
            max_total_amount_sol,
        } => {
            auto_mine_optimized(
                rpc,
                payer,
                MiningStrategy::Threshold {
                    threshold_sol,
                    amount_sol,
                    min_squares,
                    pick_squares,
                    start_before_seconds,
                    remaining_slots,
                    max_total_amount_sol,
                },
            )
            .await?;
        }
        Commands::AutoOptimized {
            amount_sol,
            min_squares,
            pick_squares,
            start_before_seconds,
            remaining_slots,
        } => {
            auto_mine_optimized(
                rpc,
                payer,
                MiningStrategy::Optimized {
                    amount_sol,
                    min_squares,
                    pick_squares,
                    start_before_seconds,
                    remaining_slots,
                },
            )
            .await?;
        }
        Commands::Balance => {
            log_balance(&rpc, &payer).await?;
        }
        Commands::Claim => {
            claim(&rpc, &payer).await?;
        }
        Commands::Status => {
            show_status(&rpc, &payer).await?;
        }
        Commands::Board => {
            log_board(&rpc).await?;
        }
        Commands::Miner => {
            log_miner(&rpc, &payer).await?;
        }
    }

    Ok(())
}

/// 挖矿策略
enum MiningStrategy {
    Threshold {
        threshold_sol: f64,
        amount_sol: f64,
        min_squares: usize,
        pick_squares: usize,
        start_before_seconds: f64,
        remaining_slots: u64,
        max_total_amount_sol: f64,
    },
    Optimized {
        amount_sol: f64,
        min_squares: usize,
        pick_squares: usize,
        start_before_seconds: f64,
        remaining_slots: u64,
    },
}

/// 优化后的自动挖矿（集成 ore_refined 的优秀特性）
async fn auto_mine_optimized(
    rpc: Arc<RpcClient>,
    payer: Arc<Keypair>,
    strategy: MiningStrategy,
) -> Result<(), anyhow::Error> {
    info!("🚀 启动优化版自动挖矿程序");

    // 显示余额
    log_balance(&rpc, &payer).await?;

    // 创建实时监控系统（ore_refined 核心特性）
    let monitor = Arc::new(Monitor::new(&rpc, &payer).await?);
    Monitor::start_all(rpc.clone(), payer.clone(), monitor.clone()).await?;

    let mut last_round_id = 0u64;

    // 获取初始价格
    let (ore_price, sol_price) = get_price_with_retry(3).await?;
    info!("💰 当前价格 - ORE: ${:.4}, SOL: ${:.2}", ore_price, sol_price);

    loop {
        // 获取实时状态快照
        let snapshot = monitor.get_snapshot().await;

        // 检测新轮次
        if snapshot.board.round_id != last_round_id {
            last_round_id = snapshot.board.round_id;

            info!("🆕 新轮次 #{}", snapshot.board.round_id);
            snapshot.log_status();

            // 更新价格
            if let Ok((o, s)) = get_price_with_retry(3).await {
                info!("💰 价格更新 - ORE: ${:.4}, SOL: ${:.2}", o, s);
            }
        }

        // 检查是否到达部署时机
        let (remaining_slots_threshold, start_before_seconds) = match &strategy {
            MiningStrategy::Threshold {
                remaining_slots,
                start_before_seconds,
                ..
            } => (*remaining_slots, *start_before_seconds),
            MiningStrategy::Optimized {
                remaining_slots,
                start_before_seconds,
                ..
            } => (*remaining_slots, *start_before_seconds),
        };

        let time_remaining = snapshot.time_remaining();
        let slots_remaining = snapshot.slots_remaining();

        info!(
            "⏰ Round {} - 剩余 {:.2}s ({} slots)",
            snapshot.board.round_id, time_remaining, slots_remaining
        );

        // 双重条件：时间和 slot（ore_refined 的精确控制）
        if time_remaining <= start_before_seconds || slots_remaining <= remaining_slots_threshold {
            info!("✅ 触发部署条件！");

            // 获取当前轮次数据
            match get_round(&rpc, snapshot.board.round_id).await {
                Ok(round) => {
                    // 选择格子
                    let selected = select_squares(&round, &strategy)?;

                    if let Some(squares_to_deploy) = selected {
                        info!("🎯 选中格子: {:?}", squares_to_deploy);

                        // 部署（双渠道提交）
                        let amount_sol = match &strategy {
                            MiningStrategy::Threshold { amount_sol, .. }
                            | MiningStrategy::Optimized { amount_sol, .. } => *amount_sol,
                        };

                        deploy_with_dual_channel(
                            &rpc,
                            &payer,
                            &snapshot,
                            &squares_to_deploy,
                            amount_sol,
                        )
                        .await?;

                        // 等待下一轮
                        info!("⏳ 等待新轮次...");
                        tokio::time::sleep(Duration::from_secs(60)).await;
                    } else {
                        info!("⏭️ 本轮条件不满足，跳过");
                    }
                }
                Err(e) => {
                    warn!("读取 Round 失败: {:?}", e);
                }
            }
        }

        sleep(Duration::from_millis(500)).await;
    }
}

/// 选择格子（根据策略）
fn select_squares(
    round: &Round,
    strategy: &MiningStrategy,
) -> Result<Option<Vec<usize>>, anyhow::Error> {
    let all_squares: Vec<(usize, f64)> = round
        .deployed
        .iter()
        .enumerate()
        .map(|(i, &lamports)| (i, lamports_to_sol(lamports)))
        .collect();

    match strategy {
        MiningStrategy::Threshold {
            threshold_sol,
            min_squares,
            pick_squares,
            max_total_amount_sol,
            ..
        } => {
            // 首先检查总部署 SOL 是否超过限制
            let total_deployed: u64 = round.deployed.iter().sum();
            let total_sol = lamports_to_sol(total_deployed);

            if total_sol > *max_total_amount_sol {
                warn!(
                    "⏭️ 跳过部署：当前sol总量 {:.6} SOL > 限制值 {:.6} SOL",
                    total_sol, max_total_amount_sol
                );
                return Ok(None);
            }

            let mut candidates: Vec<(usize, f64)> = all_squares
                .into_iter()
                .filter(|(_, v)| *v < *threshold_sol)
                .collect();

            info!(
                "📊 阈值算法 - 总部署: {:.6} SOL (限制: {:.6} SOL), 低于阈值 {:.4} SOL 的格子: {}",
                total_sol, max_total_amount_sol, threshold_sol,
                candidates.len()
            );

            if candidates.len() >= *min_squares {
                candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let picked: Vec<usize> = candidates
                    .into_iter()
                    .take(*pick_squares)
                    .map(|(idx, _)| idx)
                    .collect();
                Ok(Some(picked))
            } else {
                Ok(None)
            }
        }
        MiningStrategy::Optimized {
            min_squares,
            pick_squares,
            ..
        } => {
            let total_deployed: u64 = round.deployed.iter().sum();
            let total_sol = lamports_to_sol(total_deployed);
            let threshold = (total_sol * 0.036) - 0.005;

            info!(
                "📊 最优化算法 - 总部署: {:.6} SOL, 阈值: {:.6} SOL",
                total_sol, threshold
            );

            let mut candidates: Vec<(usize, f64)> = all_squares
                .into_iter()
                .filter(|(_, v)| *v < threshold)
                .collect();

            if candidates.len() >= *min_squares {
                candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let picked: Vec<usize> = candidates
                    .into_iter()
                    .take(*pick_squares)
                    .map(|(idx, _)| idx)
                    .collect();
                Ok(Some(picked))
            } else {
                Ok(None)
            }
        }
    }
}

/// 双渠道部署（RPC + Jito）- ore_refined 核心特性
async fn deploy_with_dual_channel(
    rpc: &Arc<RpcClient>,
    payer: &Arc<Keypair>,
    snapshot: &MonitorSnapshot,
    squares: &[usize],
    amount_sol: f64,
) -> Result<(), anyhow::Error> {
    let amount_lamports = (amount_sol * 1e9) as u64;

    // 构建部署指令
    let mut squares_array = [false; 25];
    for &idx in squares {
        if idx < 25 {
            squares_array[idx] = true;
        }
    }

    // Checkpoint（如果需要）
    if snapshot.miner.round_id < snapshot.board.round_id {
        info!("🔄 执行 checkpoint...");
        let checkpoint_ix =
            ore_api::sdk::checkpoint(payer.pubkey(), payer.pubkey(), snapshot.miner.round_id);
        submit_transaction_with_ixs(rpc, payer, &[checkpoint_ix], DEFAULT_UNITS).await?;
    }

    let deploy_ix = ore_api::sdk::deploy(
        payer.pubkey(),
        payer.pubkey(),
        amount_lamports,
        snapshot.board.round_id,
        squares_array,
    );

    // 方式1：RPC 提交
    info!("📡 通过 RPC 提交交易...");
    let rpc_result = submit_transaction_with_ixs(rpc, payer, &[deploy_ix.clone()], DEFAULT_UNITS).await;

    if let Ok(sig) = rpc_result {
        info!("✅ RPC 提交成功: {}", sig);
    }

    // 方式2：Jito Bundle 提交（异步）
    info!("🚀 通过 Jito Bundle 提交交易...");
    let rpc_clone = rpc.clone();
    let payer_clone = payer.clone();
    tokio::spawn(async move {
        if let Err(e) = send_via_jito(&rpc_clone, &payer_clone, &[deploy_ix]).await {
            warn!("Jito 提交失败: {:?}", e);
        }
    });

    info!(
        "💰 部署完成 - {} 个格子 × {:.6} SOL = {:.6} SOL",
        squares.len(),
        amount_sol,
        amount_sol * squares.len() as f64
    );

    Ok(())
}

/// 通过 Jito 发送交易
async fn send_via_jito(
    rpc: &RpcClient,
    payer: &Keypair,
    instructions: &[solana_sdk::instruction::Instruction],
) -> Result<(), anyhow::Error> {
    let blockhash = rpc.get_latest_blockhash().await?;

    let mut all_instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(DEFAULT_UNITS as u32),
        ComputeBudgetInstruction::set_compute_unit_price(0), // Jito 不需要优先费
        jito::build_bribe_ix(&payer.pubkey(), 5000), // 5000 lamports 小费
    ];
    all_instructions.extend_from_slice(instructions);

    let transaction = VersionedTransaction::try_new(
        VersionedMessage::V0(
            v0::Message::try_compile(
                &payer.pubkey(),
                &all_instructions,
                &vec![],
                blockhash,
            )
            .unwrap(),
        ),
        &[payer],
    )
    .unwrap();

    jito::send_bundle(vec![transaction]).await?;
    Ok(())
}

/// 领取奖励
async fn claim(rpc: &RpcClient, payer: &Keypair) -> Result<(), anyhow::Error> {
    let ix_sol = ore_api::sdk::claim_sol(payer.pubkey());
    let ix_ore = ore_api::sdk::claim_ore(payer.pubkey());

    submit_transaction_with_ixs(rpc, payer, &[ix_sol, ix_ore], DEFAULT_UNITS).await?;
    info!("✅ 领取成功！");

    Ok(())
}

/// 显示状态
async fn show_status(rpc: &RpcClient, payer: &Keypair) -> Result<(), anyhow::Error> {
    log_balance(rpc, payer).await?;

    let board = get_board(rpc).await?;
    let clock = get_clock(rpc).await?;
    let miner = get_miner(rpc, payer.pubkey()).await?;

    info!("┌─────────────────────────────────────────────────────┐");
    info!("│ 📊 挖矿状态                                         │");
    info!("├─────────────────────────────────────────────────────┤");
    info!("│ Round ID: {}                                      │", board.round_id);
    info!("│ 当前 Slot: {}                              │", clock.slot);
    info!("│ 结束 Slot: {}                              │", board.end_slot);
    info!(
        "│ 剩余时间: {:.2}s                                     │",
        if board.end_slot > clock.slot {
            (board.end_slot - clock.slot) as f64 * 0.4
        } else {
            0.0
        }
    );
    info!("│ Miner Round: {}                                   │", miner.round_id);
    info!("│ Checkpoint ID: {}                                 │", miner.checkpoint_id);
    info!("└─────────────────────────────────────────────────────┘");

    Ok(())
}

/// 显示 Board 信息
async fn log_board(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let board = get_board(rpc).await?;
    let clock = get_clock(rpc).await?;

    info!("Board");
    info!("  Round ID: {}", board.round_id);
    info!("  Start slot: {}", board.start_slot);
    info!("  End slot: {}", board.end_slot);
    info!(
        "  Time remaining: {:.2}s",
        if board.end_slot > clock.slot {
            (board.end_slot - clock.slot) as f64 * 0.4
        } else {
            0.0
        }
    );

    Ok(())
}

/// 显示 Miner 信息
async fn log_miner(rpc: &RpcClient, payer: &Keypair) -> Result<(), anyhow::Error> {
    let miner = get_miner(rpc, payer.pubkey()).await?;

    info!("Miner");
    info!("  Authority: {}", miner.authority);
    info!("  Rewards SOL: {:.6}", lamports_to_sol(miner.rewards_sol));
    info!(
        "  Rewards ORE: {}",
        amount_to_ui_amount(miner.rewards_ore, TOKEN_DECIMALS)
    );
    info!(
        "  Refined ORE: {}",
        amount_to_ui_amount(miner.refined_ore, TOKEN_DECIMALS)
    );
    info!("  Round ID: {}", miner.round_id);
    info!("  Checkpoint ID: {}", miner.checkpoint_id);

    Ok(())
}
