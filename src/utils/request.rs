
/// 词汇简明信息查询
/// 
/// 返回基础信息struct
pub fn do_concise_get(q: &String) -> String {
    let mut url = String::from("http://dict.youdao.com/jsonapi?jsonversion=2&q=");
    url.push_str(q);
    url.push_str(r#"&keyfrom=deskdict.main.rt&dogVersion=1.0&dogui=json&client=deskdict&id=4af70ada38270b605&vendor=webdict_default&in=YoudaoDict_webdict_default&appVer=8.9.3.0&appZengqiang=0&abTest=2&model=LENOVO&screen=1920*1080&le=eng&"#);
    let dicts = String::from(r#"{"count":14,"dicts":[["ec","ce","cj","jc","ck","kc","cf","fc","multle","related-langs","newjc","newcj","pic_dict"],["fanyi"]]}"#);

    //对dicts参数进行编码, 如果不编码会出现400 bad request
    let dicts_encoded = url::form_urlencoded::Serializer::new(String::new()).append_pair("dicts", &dicts).finish();
    // println!("编码参数: {}", dicts_encoded);
    url.push_str(&dicts_encoded);

    let r = futures::executor::block_on(do_get(&url));
    let result = match &r {
        String => r.ok().unwrap(),
        Error => "".to_string(),
    };

    result.to_string()
}


///
/// 更全面的查询
/// 
pub fn do_full_get(q: &String) -> String {
    let mut url = String::from("http://dict.youdao.com/jsonapi?jsonversion=2&q=");
    url.push_str(q);
    url.push_str(r#"&keyfrom=deskdict.main&dogVersion=1.0&dogui=json&client=deskdict&id=4af70ada38270b604&vendor=webdict_default&in=YoudaoDict_webdict_default&appVer=8.9.3.0&appZengqiang=0&abTest=2&model=LENOVO&screen=1920*1080&le=eng&dicts="#);
    let dicts = String::from(r#"{"count":21,"dicts":[["oxfordAdvance","oxford","splongman","longman","webster","collins","collins_part","ec21","ce_new","hh","newhh","newcenturyjc"],["web_search"],["web_trans"],["special"],["ee"],["phrs"],["syno"],["rel_word"],["etym"],["typos"],["blng_sents_part","media_sents_part","auth_sents_part"],["fanyi"]]}"#);
    //对dicts参数进行编码, 如果不编码会出现400 bad request
    let dicts_encoded = url::form_urlencoded::Serializer::new(String::new()).append_pair("dicts", &dicts).finish();
    // println!("编码参数: {}", dicts_encoded);
    url.push_str(&dicts_encoded);

    let r = futures::executor::block_on(do_get(&url));
    let result = match &r {
        String => r.ok().unwrap(),
        Error => "".to_string(),
    };

    result.to_string()
}

async fn do_get(url: &String) -> Result<String, reqwest::Error> {
    // println!("请求地址: {}", url);
    let client: reqwest::Client = reqwest::Client::builder()
                        .cookie_store(true)
                        .build()?;

    let mut rb = client.get(url);
    let mut headerMap = reqwest::header::HeaderMap::new();
    headerMap.append("Host", "dict.youdao.com".parse().unwrap());
    headerMap.append("User-Agent", "Youdao Desktop Dict (Windows NT 10.0)".parse().unwrap());
    headerMap.append("Accept", "*/*".parse().unwrap());
    headerMap.append("Cookie", "DESKDICT_VENDOR=webdict_default; OUTFOX_SEARCH_USER_ID=2036004394@10.108.160.17".parse().unwrap());
    headerMap.append("Connection", "Keep-Alive".parse().unwrap());
    rb = rb.headers(headerMap);

    let jsonStr = rb.send().await?.text().await.unwrap();

    Ok(jsonStr)
}