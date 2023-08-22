use serde_json::json;
use regex::Regex;
pub fn str2json(str:&str)->serde_json::Value {
    
    // 解析输入字符串
    let re = Regex::new(r"[\[\]]").unwrap();
    let result = re.replace_all(str, "");
    let user: Vec<&str> = result.split(",").collect();

    // 创建JSON对象
    let json_obj = json!({
        "user_token": user[0].trim_matches('\"'),
        "user_id": user[1].trim_matches('\"'),
        "user_account": user[2].trim_matches('\"'),
        "user_password": user[3].trim_matches('\"'),
        "user_name": user[4].trim_matches('\"'),
        "user_gender": user[5].trim_matches('\"'),
        "user_age": user[6].trim_matches('\"'),
        "user_intro": user[7].trim_matches('\"'),
        "user_sign_date": user[8].trim_matches('\"'),
        "user_music_number": user[9].parse::<u32>().unwrap(),
    });
    
    // 将JSON对象打印出来
    println!("{}", serde_json::to_string_pretty(&json_obj).unwrap());
    json_obj
}

