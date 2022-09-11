fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src")
        .compile(
            &[
                // "contracts/common.proto",
                "contracts/instruments.proto",
                "contracts/marketdata.proto",
                "contracts/operations.proto",
                "contracts/orders.proto",
                "contracts/sandbox.proto",
                "contracts/stoporders.proto",
                "contracts/users.proto",
            ],
            &["contracts/"],
        )?;

    std::fs::rename(
        "src/tinkoff.public.invest.api.contract.v1.rs",
        "src/tinkoff.rs"
    ).unwrap();

    Ok(())
}


