/// Exercise 1
/// Load the cities data from the file "cities100k.json", and find all cities in california that have more than a million people.
///
/// To create a new project for the exercise:
/// From the terminal run:
/// > cargo new rust-course
/// > cd rust_course
/// > cargo add serde --features derive
/// > cargo add serde_json
///
/// Cities100k.json has the following structure:
/*
  {
    "datasetid": "geonames-all-cities-with-a-population-1000@public",
    "recordid": "79935cd6ad4e4b4fb035208337e0ea8b9367f55e",
    "fields": {
      "coordinates": [
        51.53443,
        9.93228
      ],
      "cou_name_en": "Germany",
      "label_en": "Germany",
      "feature_code": "PPLA3",
      "population": 122149,
      "dem": 153,
      "geoname_id": "2918632",
      "admin4_code": "03159016",
      "name": "Göttingen",
      "ascii_name": "Goettingen",
      "alternate_names": "Choettingen,Chöttingen,G'otingen,Getingen,Getingenas,Getingene,Getynga,Gjottingen,Gjotyngen,Goettingen,Gotinga,Gottinga,Gottingen,Göttingen,Nketin'nken,ZEU,gatingen,ge ting gen,gettingen,ghwtynghn,goeting-gen,gtyngn,gwtyngn,gyotingana,keiththing ngein,Γκέτινγκεν,Гетинген,Гьотинген,Гёттинген,Гётынген,Ґетінґен,Գյոթինգեն,גטינגן,غوتينغن,گوتینگن,گوٹنگن,ग्यॉटिंगन,ಗಾಟಿಂಗೆನ್,เกิททิงเงิน,გეტინგენი,ゲッティンゲン,哥廷根,괴팅겐",
      "admin1_code": "06",
      "admin3_code": "03159",
      "feature_class": "P",
      "country_code": "DE",
      "admin2_code": "00",
      "timezone": "Europe/Berlin",
      "modification_date": "2019-09-05"
    },
    "geometry": {
      "type": "Point",
      "coordinates": [
        9.93228,
        51.53443
      ]
    },
    "record_timestamp": "2022-10-10T08:00:01.602+02:00"
  }
*/

#[derive(Clone, Debug, serde::Deserialize)]
pub struct City {
    pub datasetid: String,
    pub recordid: String,
    pub fields: CityData,
    pub record_timestamp: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct CityData {
    pub coordinates: [f64; 2],
    pub cou_name_en: Option<String>,
    pub label_en: Option<String>,
    pub feature_code: String,
    pub population: i64,
    pub dem: i64,
    pub geoname_id: String,
    pub name: String,
    pub admin1_code: Option<String>,
    pub admin2_code: Option<String>,
    pub admin3_code: Option<String>,
    pub admin4_code: Option<String>,
    pub feature_class: String,
    pub country_code: String,
    pub timezone: String,
    pub modification_date: String,
}

// pub enum Result<T, E> {
//    Ok(T),
//    Err(E),
// }
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //    let data = match std::fs::read_to_string("cities100k.json") {
    //        Ok(x) => x,
    //        Err(e) => {
    //            eprintln!("{:?}", e);
    //            return Err(e);
    //        },
    //    };

    Ok(())
}

