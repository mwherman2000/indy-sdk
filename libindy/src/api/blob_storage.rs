extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::blob_storage::BlobStorageCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;

#[no_mangle]
pub extern fn indy_open_reader(command_handle: i32,
                               type_: *const c_char,
                               config_json: *const c_char,
                               location: *const c_char,
                               hash: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, handle: i32)>) -> ErrorCode {
    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(config_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(location, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(hash, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::BlobStorage(BlobStorageCommand::OpenReader(
            type_,
            config_json,
            location,
            hash,
            Box::new(move |result| {
                let (err, handle) = result_to_err_code_1!(result, 0);
                cb(command_handle, err, handle)
            })
        )));

    result_to_err_code!(result)
}