fn main() {
    println!("Hello, world!");
}
use std::fs::OpenOptions;
// 標準入力を扱う為にインポート
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use budget_app::services;


// ファイル入出力を行うためインポート
use std::fs::File;
use std::io::Write;
use std::io::BufWriter;
use std::io::Read;
use std::io::Result;

// JSON形式への返還とファイルIO用クレートのインポート
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
//use serde::Serialize;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


// 品目データ格納用構造体定義
#[derive(Serialize, Deserialize, Debug)]
struct Record{
    _goods:         String,
    _category:      String,
    _price:         i32,
    _date:          String,
}

impl Record {
    fn new() -> Record {
        Record {
            _goods:     String::new(),
            _category:  String::new(),
            _price:     0,
            _date:      String::new(),
         }
    }
    fn push(&mut self, _goods: String, _category: String, _price: i32, _data: String) {

        self._goods.push(_goods);
        self._category.push(_category);
        self._price.push(_price);
        self._data.push(_data);
    }
}

//impl Serialize for Record {
//    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//    where
//        S: serde::Serializer,
//    {
//        let mut state = serializer.serialize_struct("Record", 4)?;
//        state.serialize_field("_goods", &self._goods)?;
//        state.serialize_field("_category", &self._category)?;
//        state.serialize_field("_price", &self._price)?;
//        state.serialize_field("_date", &self._date)?;
//        state.end()
//    }
//}


fn main(){

  //  入出力用ファイル名を定義
  //  let mut file = File::create("output.txt")?;


    //let service_type :i32;
    let mut service_type = String::new();
    println!("実行したい内容を入力してください（0:登録、1:集計）");
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("整数で入力してください");
    services::validate::InputValidator::validate_service_type(service_type);
    
    //service_type = get_user_input("実行したい内容を入力してください（0:登録、1:集計）");

    match service_type {
            0 => {
                println!("収支を登録します");

               let _menu1 :i32 = get_user_input("登録種別を入力してください（0:収入、1:支出)");
               match _menu1 {
                           0 => {


                                // 構造体単体でのファイル追記ができない。（仕様上できない）
                                // 一度ファイルを構造体に読み込み、pushしたあとで、構造体としてJSON形式で追記する必要がある。
                                // JSON形式のファイルへは、上書きしかできない

                                // ファイルを開く
                                let file_input = File::open("output.json").expect("Filed to open file.");

                                let reader = BufReader::new(file_input);
                                let read_record: Record = serde_json::from_reader(reader).expect("Failed to parse json");


                                // debag
                                println!("{:?}",read_record);


                                //let add_record = Record {
                                    let add_goods = get_user_param("品目名を入力してください");
                                    let add_category = get_user_category("カテゴリーを入力してください(0:食費、1:趣味、2:その他）");
                                    let add_price = get_user_param_toInt(_menu1, "金額を入力してください");
                                    let add_date = get_user_param("日付を入力してください(yyyy-mm-dd)");
                                //};

                                read_record.push(add_goods, add_category, add_price, add_date);
                                
                                //let _goods = get_user_param("品目名を入力してください");
                                //let _category = get_user_category("カテゴリーを入力してください(0:食費、1:趣味、2:その他）");
                                //let _price = get_user_param("金額を入力してください");
                                //let _date = get_user_param("日付を入力してください(yyyy-mm-dd)");

                                //Item { name:"飲み会" , category: Expence(Food), price: 3000, date: 2022-04-01 };
                                //println!(" name: \" {} \", category: Expence({}), price: {}, date: {} ", record._goods, record._category, record._price, record._date);
                                
                                // 登録情報をファイルに出力する
                                // JSONデータに変換する
                                ////let json_record = serde_json::to_string(&record).unwrap();

                                // JSONデータをファイルに書き込む
                                ////let file_path = "output.json";
                                ////let mut file = File::create(file_path).unwrap();

                                //let mut file = OpenOptions::new()
                                //    .append(true)
                                //    .create(true)
                                //    .open(file_path);


                                ////file.write_all(json_record.as_bytes()).unwrap();

                                println!("項目の登録が完了しました！");
                
                            },
                            1 => {

                                let record = Record {
                                    _goods: get_user_param("品目名を入力してください"),
                                    _category: get_user_category("カテゴリーを入力してください(0:食費、1:趣味、2:その他）"),
                                    _price: get_user_param_toInt(_menu1, "金額を入力してください"),
                                    _date: get_user_param("日付を入力してください(yyyy-mm-dd)"),
                                };

                                //let native_price = -record._price;
                                //record._price=native_price;
                                //let _goods = get_user_param("品目名を入力してください");
                                //let _category = get_user_category("カテゴリーを入力してください(0:食費、1:趣味、2:その他）");
                                //let _price = get_user_param("金額を入力してください");
                                //let _date = get_user_param("日付を入力してください(yyyy-mm-dd)");

                                //Item { name:"飲み会" , category: Expence(Food), price: 3000, date: 2022-04-01 }
                                //println!(" name: \" {} \", category: Expence({}), price: {}, date: {} ", _goods, _category, _price, _date);
                                
                                // 登録情報をファイルに出力する
                                // JSONデータに変換する
                                let json_record = serde_json::to_string(&record).unwrap();

                                // JSONデータをファイルに書き込む
                                let file_path = "output.json";
                                let mut file = File::create(file_path).unwrap();
                                file.write_all(json_record.as_bytes()).unwrap();

                                println!("項目の登録が完了しました！");
                
                            },
                            _ => println!("入力値異常です。プログラムを終了します。"),
                }



            }
            1 =>  println!("家計簿の集計を行います"),
            _  =>  println!("入異常です。プログラムを終了します。"),
    }    

}


fn get_user_input(prompt: &str) -> i32{

    let mut input = String::new();

    println!("{}", prompt);

    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

    let _menu: i32 = match input.trim().parse(){
        Ok(value) => return value,
        Err(_) => {
            println!("正しい値を入力してください");
            return 9;
        }
    };

}

fn get_user_param(prompt: &str) -> String{

    let mut input = String::new();

    println!("{}", prompt);

    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

    return input;
    //let _menu: String = match input{
    //    Ok(value) => return value,
    //    Err(_) => {
    //        println!("正しい値を入力してください");
            //return value;
    //    }
    //};

}

fn get_user_param_toInt(_menu1: i32, prompt: &str) -> i32{

    let mut input = String::new();
    let mut native_num = 0;

    println!("{}", prompt);

    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

        //println!("{}", _menu1);
        match input.trim().parse::<i32>() {
            //Ok(num) => return num,
            
            Ok(num) => 
            match _menu1{
                0=> return num,
                1=> {native_num = -num;
                    return native_num
                    },
                _=> panic!("整数を入力してください"),
            }            
            Err(_) => panic!("Invalid input. Please enter a valid number."),
        }

    //return input;
    //let _menu: String = match input{
    //    Ok(value) => return value,
    //    Err(_) => {
    //        println!("正しい値を入力してください");
            //return value;
    //    }
    //};

}

fn get_user_category(prompt: &str) -> String{

    let mut input = String::new();
    let category_1 = "食費";
    let category_2 = "趣味";
    let category_3 = "その他";

    println!("{}", prompt);

    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

    let _category: i32 = match input.trim().parse(){
        Ok(_category) => {

        match _category{
            0 => return category_1.to_string(),
            1 => return category_2.to_string(),
            _ => return category_3.to_string(),
        }
        }
        Err(_) => {
            println!("正しい値を入力してください");
            return "Err".to_string();
        }
    };

}