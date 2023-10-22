use petompp_web_models::models::password_requirements::PasswordRequirements;
use strum::EnumIter;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, EnumIter)]
pub enum TK {
    __version,
    Home,
    Projects,
    About,
    Contact,
    Ok,
    Cancel,
    Save,
    SaveChanges,
    SaveChangesQuestion,
    Discard,
    DiscardChanges,
    DiscardChangesQuestion,
    Logout,
    LogoutQuestion,
    Login,
    Register,
    Username,
    TypeUsername,
    Password,
    TypePassword,
    UserManagement,
    Id,
    Name,
    Actions,
    Activate,
    ActivateUserQuestion(String),
    Delete,
    DeleteUserQuestion(String),
    ProjectsDescription,
    Edit,
    Editor,
    Preview,
    SaveDraft,
    Blog,
    BackToBlogPosts,
    Created,
    Updated,
    ErrorOccured,
    E_Auth_MissingClaim(String),
    E_Auth_InvalidFormat(String),
    E_Auth_TokenExpiredS(i32),
    E_Auth_JwtError(String),
    E_Database(String),
    E_DatabaseConnection(String),
    E_UserNameTaken(String),
    E_UserNotFound(String),
    E_InvalidCredentials,
    E_UserNotConfirmed(String),
    E_Validation_Username_InvalidLength(i32, i32),
    E_Validation_Username_InvalidCharacters(String),
    E_Validation_Password(PasswordRequirements),
    E_Validation_PasswordRequirement(i32, i32, String),
    E_Validation_PasswordRequirement_ContainsLowercase,
    E_Validation_PasswordRequirement_ContainsUppercase,
    E_Validation_PasswordRequirement_ContainsNumber,
    E_Validation_PasswordRequirement_ContainsSpecialCharacter,
    E_Validation_Country,
    E_Validation_Query_InvalidColumn(String),
    E_Validation_ResourceData_KeyMismatch(String, String),
    E_Validation_ResourceData_KeyMissing,
    E_Validation_ResourceData_ValueMissing,
}

impl std::fmt::Display for TK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = format!("{:?}", self);
        if let Some(pos) = val.find('(') {
            write!(f, "{}", &val[..pos])
        } else {
            write!(f, "{}", val)
        }
    }
}
