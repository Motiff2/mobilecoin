// Copyright (c) 2018-2022 The MobileCoin Foundation

//! Build script for the ingest_enclave_measurement crate.

use cargo_emit::{rerun_if_env_changed, rustc_cfg};
use mc_util_build_enclave::Builder;
use mc_util_build_script::Environment;
use mc_util_build_sgx::{SgxEnvironment, SgxMode, TcsPolicy};
use std::{env::var, path::PathBuf};

// Changing this version is a breaking change, you must update the crate version
// if you do.
const SGX_VERSION: &str = "2.21.100.1";

const INGEST_ENCLAVE_PRODUCT_ID: u16 = 4;
const INGEST_ENCLAVE_SECURITY_VERSION: u16 = 7;
const INGEST_ENCLAVE_NAME: &str = "ingest-enclave";
const INGEST_ENCLAVE_DIR: &str = "../trusted";

fn main() {
    let env = Environment::default();
    let sgx = SgxEnvironment::new(&env).expect("Could not read SGX environment");

    if sgx.sgx_mode() == SgxMode::Simulation {
        rustc_cfg!("feature=\"sgx-sim\"");
    }

    let mut builder = Builder::new(
        &env,
        &sgx,
        SGX_VERSION,
        INGEST_ENCLAVE_NAME,
        INGEST_ENCLAVE_DIR.as_ref(),
    )
    .expect("Could not construct builder");

    rerun_if_env_changed!("INGEST_ENCLAVE_CSS");
    if let Ok(value) = var("INGEST_ENCLAVE_CSS") {
        builder.css(PathBuf::from(&value));
    }

    rerun_if_env_changed!("INGEST_ENCLAVE_UNSIGNED");
    if let Ok(value) = var("INGEST_ENCLAVE_UNSIGNED") {
        builder.unsigned_enclave(PathBuf::from(&value));
    }

    rerun_if_env_changed!("INGEST_ENCLAVE_SIGNED");
    if let Ok(value) = var("INGEST_ENCLAVE_SIGNED") {
        builder.signed_enclave(PathBuf::from(&value));
    }

    rerun_if_env_changed!("INGEST_ENCLAVE_LDS");
    if let Ok(value) = var("INGEST_ENCLAVE_LDS") {
        builder.lds(PathBuf::from(&value));
    }

    rerun_if_env_changed!("INGEST_ENCLAVE_PRIVKEY");
    if let Ok(value) = var("INGEST_ENCLAVE_PRIVKEY") {
        builder.privkey(PathBuf::from(&value));
    }

    rerun_if_env_changed!("INGEST_ENCLAVE_GENDATA");
    rerun_if_env_changed!("INGEST_ENCLAVE_PUBKEY");
    rerun_if_env_changed!("INGEST_ENCLAVE_SIGNATURE");
    if let Ok(gendata) = var("INGEST_ENCLAVE_GENDATA") {
        if let Ok(pubkey) = var("INGEST_ENCLAVE_PUBKEY") {
            if let Ok(signature) = var("INGEST_ENCLAVE_SIGNATURE") {
                builder.catsig(gendata.into(), pubkey.into(), signature.into());
            }
        }
    }

    builder
        .target_dir(env.target_dir().join(INGEST_ENCLAVE_NAME).as_path())
        .config_builder
        .debug(sgx.sgx_mode() == SgxMode::Simulation || env.profile() != "release")
        .prod_id(INGEST_ENCLAVE_PRODUCT_ID)
        .isv_security_version(INGEST_ENCLAVE_SECURITY_VERSION)
        .tcs_num(32)
        .tcs_min_pool(1)
        .tcs_policy(TcsPolicy::Unbound)
        .stack_max_size(256 * 1024)
        .heap_max_size(256 * 1024 * 1024);

    let _sig = builder
        .build()
        .expect("Failed to extract ingest-enclave signature");
}
