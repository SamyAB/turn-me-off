use axum::http::StatusCode;

/// Checks if the turn-me-off server is alive.
#[utoipa::path(get, path = "/alive", responses((status = 200, body = String, description = "Alive message")))]
pub async fn alive() -> (StatusCode, &'static str) {
    (StatusCode::OK, "turn-me-off is alive")
}

/// Turns off the machine on which this HTTP server runs.
#[utoipa::path(
        put,
        path = "/turn-off",
        responses(
            (status = 200, body = String, description = "Turn off message"),
            (status = 500, body = String, description = "Command failed message"),
            (status = 501, body = String, description = "Command failed message"),
        ),
    )]
pub async fn turn_off() -> (StatusCode, &'static str) {
    let poweroff_output = tokio::process::Command::new("/usr/bin/systemctl")
        .arg("poweroff")
        .arg("--no-ask-password")
        .output()
        .await
        .expect("Unable to run a process with the command systemctl");

    match poweroff_output.status.code() {
        Some(0) => (StatusCode::OK, "This machine will now turn off.\n"),
        Some(1) => {
            let error_message = String::from_utf8(poweroff_output.stderr)
                .expect("The error messages from systemctl do not contain non UTF-8 characters");
            if error_message.contains("Interactive authentication required.") {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "The user running turn-me-off does not have the permission to shutdown the device.\n",
                )
            } else {
                (
                    StatusCode::NOT_IMPLEMENTED,
                    "Shutdown command failed for unknown reason, status code 1.\n",
                )
            }
        }
        Some(_) => (
            StatusCode::NOT_IMPLEMENTED,
            "Shutdown command failed for unknown reason.\n",
        ),
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unexpected error occurred while running the shutdown command, please file an issue.\n",
        ),
    }
}

/// Reboots the machine on which this HTTP server runs.
#[utoipa::path(
        put,
        path = "/reboot",
        responses(
            (status = 200, body = String, description = "Turn off message"),
            (status = 500, body = String, description = "Command failed message"),
            (status = 501, body = String, description = "Command failed message"),
        ),
    )]
pub async fn reboot() -> (StatusCode, &'static str) {
    let poweroff_output = tokio::process::Command::new("/usr/bin/systemctl")
        .arg("reboot")
        .arg("--no-ask-password")
        .output()
        .await
        .expect("Unable to run a process with the command systemctl");

    match poweroff_output.status.code() {
        Some(0) => (StatusCode::OK, "This machine will now reboot.\n"),
        Some(1) => {
            let error_message = String::from_utf8(poweroff_output.stderr)
                .expect("The error messages from systemctl do not contain non UTF-8 characters");
            if error_message.contains("Interactive authentication required.") {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "The user running turn-me-off does not have the permission to reboot the device.\n",
                )
            } else {
                (
                    StatusCode::NOT_IMPLEMENTED,
                    "Reboot command failed for unknown reason, status code 1.\n",
                )
            }
        }
        Some(_) => (
            StatusCode::NOT_IMPLEMENTED,
            "Reboot command failed for unknown reason.\n",
        ),
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unexpected error occurred while running the reboot command, please file an issue.\n",
        ),
    }
}

/// Get the hostname of the device on which turn-me-off is running
#[utoipa::path(get, path = "/hostname", responses((status = 200, body = String, description = "Hostname of the device")))]
pub async fn hostname() -> (StatusCode, String) {
    let host_name = tokio::fs::read_to_string("/etc/hostname")
        .await
        .expect("We should be able to read the content of /etc/hostname");

    (StatusCode::OK, host_name)
}

/// Put system to sleep
#[utoipa::path(
        put,
        path = "/suspend",
        responses(
            (status = 200, body = String, description = "Suspend message"),
            (status = 500, body = String, description = "Command failed message"),
            (status = 501, body = String, description = "Command failed message"),
        ),
    )]
pub async fn suspend() -> (StatusCode, &'static str) {
    let suspend_output = tokio::process::Command::new("/usr/bin/systemctl")
        .arg("suspend")
        .arg("--no-ask-password")
        .output()
        .await
        .expect("Unable to run a process with the command systemctl");

    match suspend_output.status.code() {
        Some(0) => (StatusCode::OK, "This machine will now enter sleep mode.\n"),
        Some(1) => {
            let error_message = String::from_utf8(suspend_output.stderr)
                .expect("The error messages from systemctl do not contain non UTF-8 characters");
            if error_message.contains("Interactive authentication required.") {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "The user running turn-me-off does not have the permission to suspend the device.\n",
                )
            } else {
                (
                    StatusCode::NOT_IMPLEMENTED,
                    "Suspend command failed for unknown reason, status code 1.\n",
                )
            }
        }
        Some(_) => (
            StatusCode::NOT_IMPLEMENTED,
            "Reboot command failed for unknown reason.\n",
        ),
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unexpected error occurred while running the reboot command, please file an issue.\n",
        ),
    }
}
