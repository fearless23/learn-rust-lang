/*
  Enums - Collection of similar and constant values
  ex: Error Types
  ex: HTTP Codes
  ex: Roles
  ex: Math constants
  ex: Keyboard Keycodes
  ex: Time -> secs,mins,hours
  ex: Directions
*/

enum _Movement {
  Up,
  Down,
  Left,
  Right,
}

enum _Roles {
  Admin,
  User,
  Editor,
}

enum FamilyMembers {
  Mom,
  Dad,
  _Wife,
  Daughter,
}

fn who_is_this(p: FamilyMembers) {
  match p {
    FamilyMembers::Dad => println!("paisa bhej do..."),
    FamilyMembers::Mom => println!("khana kha liya..."),
    FamilyMembers::_Wife => println!("hey sweetheart..."),
    FamilyMembers::Daughter => println!("aw e ovv..."),
  }
}

pub fn run() {
  who_is_this(FamilyMembers::Dad);
  who_is_this(FamilyMembers::Mom);
  who_is_this(FamilyMembers::_Wife);
  who_is_this(FamilyMembers::Daughter);
}
