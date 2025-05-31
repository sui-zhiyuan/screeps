use std::sync::Once;
use tracing::{Level, info};
use tracing_wasm::{ConsoleConfig, WASMLayerConfigBuilder};

static CELL: Once = Once::new();

pub fn init() {
    CELL.call_once(initialize);
}

fn initialize() {
    let mut config_builder = WASMLayerConfigBuilder::new();
    config_builder.set_console_config(ConsoleConfig::ReportWithConsoleColor);
    config_builder.set_max_level(Level::INFO);
    tracing_wasm::set_as_global_default_with_config(config_builder.build());
    info!("initialize trace finish");
}
