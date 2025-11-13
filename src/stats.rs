use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use chrono::Local;

/// 单轮历史信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoundHistory {
    pub round_id: u64,
    pub timestamp: String,
    pub status: String, // "deployed" or "skipped"
    pub deployed_sol: f64,
    pub gained_sol: f64,
    pub gained_ore: f64,
    pub result: String, // "success" or "failure" or "skipped"
    pub history: HistoryStats,
}

/// 历史统计信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryStats {
    pub win_rate: f64,           // 胜率 (%)
    pub total_ore: f64,          // 总获得ORE
    pub total_deployed_sol: f64, // 总消耗SOL
    pub total_gained_sol: f64,   // 总获得SOL
    pub profit_loss_ratio: f64,  // 盈亏比（ore / (deployed_sol - gained_sol)）
}

/// 奖励历史数据库
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RewardDatabase {
    pub rounds: HashMap<String, RoundHistory>,
}

impl RewardDatabase {
    /// 加载或创建奖励数据库
    pub fn load_or_create(file_path: &str) -> Self {
        if Path::new(file_path).exists() {
            match fs::read_to_string(file_path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(db) => return db,
                        Err(e) => {
                            eprintln!("Failed to parse reward.json: {}", e);
                            return Self::new();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read reward.json: {}", e);
                    return Self::new();
                }
            }
        }
        Self::new()
    }

    /// 创建新的数据库
    pub fn new() -> Self {
        Self {
            rounds: HashMap::new(),
        }
    }

    /// 保存到文件
    pub fn save(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(file_path, json)?;
        Ok(())
    }

    /// 添加或更新轮次记录
    pub fn add_or_update_round(
        &mut self,
        round_id: u64,
        timestamp: String,
        status: String,
        deployed_sol: f64,
        gained_sol: f64,
        gained_ore: f64,
        result: String,
    ) {
        let history = self.calculate_history(deployed_sol, gained_sol, gained_ore);

        let key = format!("round_{}", round_id);
        self.rounds.insert(
            key,
            RoundHistory {
                round_id,
                timestamp,
                status,
                deployed_sol,
                gained_sol,
                gained_ore,
                result,
                history,
            },
        );
    }

    /// 计算历史统计数据（包括当前轮次和历史数据）
    fn calculate_history(&self, deployed_sol: f64, gained_sol: f64, gained_ore: f64) -> HistoryStats {
        let mut total_ore = 0.0;
        let mut total_deployed_sol = 0.0;
        let mut total_gained_sol = 0.0;
        let mut win_count = 0u64;
        let mut total_count = 0u64;

        // 累计历史数据（不包括当前轮次）
        for history in self.rounds.values() {
            if history.status == "deployed" {
                total_ore += history.gained_ore;
                total_deployed_sol += history.deployed_sol;
                total_gained_sol += history.gained_sol;

                if history.result == "success" {
                    win_count += 1;
                }
                total_count += 1;
            }
        }

        // 加上当前轮次
        if deployed_sol > 0.0 {
            total_ore += gained_ore;
            total_deployed_sol += deployed_sol;
            total_gained_sol += gained_sol;
            total_count += 1;
            if gained_sol > 0.0 {
                win_count += 1;
            }
        }

        let win_rate = if total_count > 0 {
            (win_count as f64 / total_count as f64) * 100.0
        } else {
            0.0
        };

        // 计算盈亏比：ore / (deployed_sol - gained_sol)
        let cost_sol = total_deployed_sol - total_gained_sol;
        let profit_loss_ratio = if cost_sol > 0.0 {
            total_ore / cost_sol
        } else {
            0.0
        };

        HistoryStats {
            win_rate,
            total_ore,
            total_deployed_sol,
            total_gained_sol,
            profit_loss_ratio,
        }
    }

    /// 获取历史统计（不包括当前轮次）
    pub fn get_history_stats(&self) -> HistoryStats {
        let mut total_ore = 0.0;
        let mut total_deployed_sol = 0.0;
        let mut total_gained_sol = 0.0;
        let mut win_count = 0u64;
        let mut total_count = 0u64;

        for history in self.rounds.values() {
            if history.status == "deployed" {
                total_ore += history.gained_ore;
                total_deployed_sol += history.deployed_sol;
                total_gained_sol += history.gained_sol;

                if history.result == "success" {
                    win_count += 1;
                }
                total_count += 1;
            }
        }

        let win_rate = if total_count > 0 {
            (win_count as f64 / total_count as f64) * 100.0
        } else {
            0.0
        };

        let cost_sol = total_deployed_sol - total_gained_sol;
        let profit_loss_ratio = if cost_sol > 0.0 {
            total_ore / cost_sol
        } else {
            0.0
        };

        HistoryStats {
            win_rate,
            total_ore,
            total_deployed_sol,
            total_gained_sol,
            profit_loss_ratio,
        }
    }

    /// 获取上一轮的信息
    pub fn get_last_round(&self) -> Option<RoundHistory> {
        self.rounds
            .values()
            .max_by_key(|h| h.round_id)
            .cloned()
    }
}

/// ANSI 颜色代码
pub struct Colors;

impl Colors {
    pub const GREEN: &'static str = "\x1b[32m";
    pub const RED: &'static str = "\x1b[31m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const RESET: &'static str = "\x1b[0m";
}

/// 生成轮次报告
pub fn generate_round_report(
    database: &RewardDatabase,
    round_id: u64,
    deployed_sol: f64,
    previous_rewards_sol: f64,
    current_rewards_sol: f64,
    previous_rewards_ore: f64,
    current_rewards_ore: f64,
) -> String {
    let gained_sol = current_rewards_sol - previous_rewards_sol;
    let gained_ore = current_rewards_ore - previous_rewards_ore;
    let is_deployed = deployed_sol > 0.0;
    let is_success = gained_sol > 0.0;

    // 新轻逻辑分支处理
    let mut report = String::new();

    if is_deployed {
        // 部署的情况
        let status_color = if is_success { Colors::GREEN } else { Colors::RED };
        let result_text = if is_success { "成功" } else { "失败" };

        report.push_str(&format!(
            "\n{}┌─────────────────────────────────────────────────────┐{}\n",
            Colors::CYAN, Colors::RESET
        ));
        report.push_str(&format!(
            "{}│ 📊 上轮信息 (Round #{})                          │{}\n",
            Colors::CYAN, round_id - 1, Colors::RESET
        ));
        report.push_str(&format!(
            "{}├─────────────────────────────────────────────────────┤{}\n",
            Colors::CYAN, Colors::RESET
        ));
        report.push_str(&format!(
            "{}│ 状态: {}部署{}                                    │{}\n",
            Colors::CYAN, Colors::GREEN, Colors::RESET, Colors::RESET
        ));
        report.push_str(&format!(
            "{}│ 结果: {}{}{} (SOL+{:.6}, ORE+{:.2})                 │{}\n",
            Colors::CYAN,
            status_color,
            result_text,
            Colors::RESET,
            gained_sol,
            gained_ore,
            Colors::RESET
        ));
        report.push_str(&format!(
            "{}│ 上轮部署: {:.6} SOL                            │{}\n",
            Colors::CYAN, deployed_sol, Colors::RESET
        ));
        report.push_str(&format!(
            "{}│ 上轮获得: {:.6} SOL, {:.2} ORE                   │{}\n",
            Colors::CYAN, gained_sol, gained_ore, Colors::RESET
        ));
        report.push_str(&format!(
            "{}├─────────────────────────────────────────────────────┤{}\n",
            Colors::CYAN, Colors::RESET
        ));
    } else {
        // 跳过的情况
        report.push_str(&format!(
            "\n{}┌─────────────────────────────────────────────────────┐{}\n",
            Colors::CYAN, Colors::RESET
        ));
        report.push_str(&format!(
            "{}│ 📊 上轮信息 (Round #{})                          │{}\n",
            Colors::CYAN, round_id - 1, Colors::RESET
        ));
        report.push_str(&format!(
            "{}├─────────────────────────────────────────────────────┤{}\n",
            Colors::CYAN, Colors::RESET
        ));
        report.push_str(&format!(
            "{}│ 状态: {}跳过{}                                    │{}\n",
            Colors::CYAN, Colors::YELLOW, Colors::RESET, Colors::RESET
        ));
        report.push_str(&format!(
            "{}├─────────────────────────────────────────────────────┤{}\n",
            Colors::CYAN, Colors::RESET
        ));
    }

    // 历史统计信息
    let history = database.get_history_stats();
    report.push_str(&format!(
        "{}│ 📈 历史统计                                         │{}\n",
        Colors::CYAN, Colors::RESET
    ));
    report.push_str(&format!(
        "{}├─────────────────────────────────────────────────────┤{}\n",
        Colors::CYAN, Colors::RESET
    ));
    report.push_str(&format!(
        "{}│ 胜率: {:.2}%                                        │{}\n",
        Colors::CYAN, history.win_rate, Colors::RESET
    ));
    report.push_str(&format!(
        "{}│ 总获得ORE: {:.2}                                    │{}\n",
        Colors::CYAN, history.total_ore, Colors::RESET
    ));
    report.push_str(&format!(
        "{}│ 总消耗SOL: {:.6}                                  │{}\n",
        Colors::CYAN, history.total_deployed_sol, Colors::RESET
    ));
    report.push_str(&format!(
        "{}│ 总获得SOL: {:.6}                                  │{}\n",
        Colors::CYAN, history.total_gained_sol, Colors::RESET
    ));

    let profit_text = if history.profit_loss_ratio >= 0.0 {
        format!("{}+{:.4}{}", Colors::GREEN, history.profit_loss_ratio, Colors::RESET)
    } else {
        format!("{}{:.4}{}", Colors::RED, history.profit_loss_ratio, Colors::RESET)
    };
    report.push_str(&format!(
        "{}│ 盈亏比: {} ORE/SOL                                  │{}\n",
        Colors::CYAN, profit_text, Colors::RESET
    ));
    report.push_str(&format!(
        "{}└─────────────────────────────────────────────────────┘{}\n",
        Colors::CYAN, Colors::RESET
    ));

    report
}
