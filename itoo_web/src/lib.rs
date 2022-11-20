use config::log::init_log;
use initialize::config::init_config;
use log::info;
use state::Container;

pub mod config;
pub mod initialize;
/**
 * 整个项目的上下文管理
 * 包括：
 *      配置文件
 *      数据库连接池
 */
pub static CONTEXT_MANAGER: Container![Send + Sync] = <Container![Send + Sync]>::new();

pub async fn init_context() {
    //加载配置
    init_config().await;

    //日志
    init_log();

    info!("ConfigContext init complete");
}
