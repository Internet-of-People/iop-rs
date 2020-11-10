use super::*;

#[no_mangle]
pub extern "C" fn delete_ValidationResult(validation: *mut ValidationResult) {
    delete(validation)
}

#[no_mangle]
pub extern "C" fn ValidationResult_status_get(
    validation: *mut ValidationResult,
) -> *mut raw::c_char {
    let validation = unsafe { convert::borrow_in(validation) };
    let status = validation.status().to_string();
    convert::string_out(status)
}

#[no_mangle]
pub extern "C" fn ValidationResult_issues_get(
    validation: *mut ValidationResult,
) -> *mut CSlice<*mut ValidationIssue> {
    let validation = unsafe { convert::borrow_in(validation) };
    let issues = validation.issues();
    let issues = issues.iter().cloned().map(convert::move_out).collect::<Box<[_]>>();
    convert::move_out(CSlice::from(issues))
}

#[no_mangle]
pub extern "C" fn delete_ValidationIssue(issue: *mut ValidationIssue) {
    delete(issue)
}

#[no_mangle]
pub extern "C" fn ValidationIssue_code_get(issue: *mut ValidationIssue) -> u32 {
    let issue = unsafe { convert::borrow_in(issue) };
    issue.code()
}

#[no_mangle]
pub extern "C" fn ValidationIssue_reason_get(issue: *mut ValidationIssue) -> *mut raw::c_char {
    let issue = unsafe { convert::borrow_in(issue) };
    let reason = issue.reason().to_owned();
    convert::string_out(reason)
}

#[no_mangle]
pub extern "C" fn ValidationIssue_severity_get(issue: *mut ValidationIssue) -> *mut raw::c_char {
    let issue = unsafe { convert::borrow_in(issue) };
    let severity = issue.severity().to_string();
    convert::string_out(severity)
}
