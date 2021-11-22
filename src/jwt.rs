


pub mod jwt {
  use either::*;
  use regex::Regex;
  use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
  use anyhow::anyhow;
  use anyhow::Result;
  use neon::prelude::*;
  use serde::Serialize;
  use serde::Deserialize;

  pub type AnyError = anyhow::Error;

  #[derive(Serialize, Deserialize, Default, Debug)]
  pub struct Claims {
    secret: Option<String>,
    aud: Option<String>,
    exp: u64,
    iat: Option<u64>,
    iss: Option<String>,
    nbf: Option<u64>,
    sub: Option<String>,
    algorithm: Option<Algorithm>
  }

  impl Claims {
    pub(crate) fn setJsObjectByStringType(&self, mut cx: &mut FunctionContext, obj: &mut Handle<JsObject>, claimsKey: &Option<String>, key: &str) {
      match claimsKey {
        Some(val) => {
          let val = cx.string(val);
          obj.set(cx, key, val);
        },
        None => {
          let  null = cx.null();
          obj.set(cx, key, null);
        },
      };
    }
    pub(crate) fn setJsObjectByNumberType(&self, mut cx: &mut FunctionContext, obj: &mut Handle<JsObject>, claimsKey: Option<u64>, key: &str) {
      match claimsKey {
        Some(val) => {
          let val = cx.number(val as f64);
          obj.set(cx, key, val);
        },
        None => {
          let  null = cx.null();
          obj.set(cx, key, null);
        },
      };
    }
    pub(crate) fn setJsObjectByAlgorithmType(&self, mut cx: &mut FunctionContext, obj: &mut Handle<JsObject>, claimsKey: Option<Algorithm>, key: &str) {
      match claimsKey {
        Some(val) => {
          let val = match val {
             Algorithm::HS256 => "HS256",
             Algorithm::HS384 => "HS384",
             Algorithm::HS512 => "HS512",
             Algorithm::ES256 => "ES256",
             Algorithm::ES384 => "ES384",
             Algorithm::RS256 => "RS256",
             Algorithm::RS384 => "RS384",
             Algorithm::RS512 => "RS512",
             Algorithm::PS256 => "PS256",
             Algorithm::PS384 => "PS384",
             Algorithm::PS512 => "PS512",
            _ => "HS256",
          };
          let val = cx.string(val);
          obj.set(cx, key, val);
        },
        None => {
          let  null = cx.null();
          obj.set(cx, key, null);
        },
      };
    }
    pub(crate) fn toJsObject<'a>(&self, mut cx: &mut FunctionContext<'a>) -> Option<Handle<'a, JsObject>> {
      let mut obj = cx.empty_object();
      self.setJsObjectByStringType(&mut cx, &mut obj, &self.secret, "secret");
      self.setJsObjectByStringType(&mut cx, &mut obj, &self.aud, "aud");
      self.setJsObjectByAlgorithmType(&mut cx, &mut obj, self.algorithm, "algorithm");
      self.setJsObjectByStringType(&mut cx, &mut obj, &self.sub, "sub");
      self.setJsObjectByStringType(&mut cx, &mut obj, &self.iss, "iss");
      self.setJsObjectByNumberType(&mut cx, &mut obj, self.iat, "iat");
      self.setJsObjectByNumberType(&mut cx, &mut obj, self.nbf, "nbf");
      let exp = cx.number(self.exp as f64);
      obj.set(cx, "exp", exp);

      Option::from(obj)
    }
    pub(crate) fn getString(mut cx: &mut FunctionContext, jsValue: Handle<JsValue>) -> String {
        jsValue.to_string(cx).unwrap().value(cx)
    }
    pub(crate) fn getOptionString(mut cx: &mut FunctionContext, jsValue: Handle<JsValue>) -> Option<String> {
      if jsValue.is_a::<JsString, _>(cx) {
        return Option::from(Claims::getString(cx, jsValue));
      }
      return None;
    }
    pub(crate) fn getOptionNumber(mut cx: &mut FunctionContext, jsValue: Handle<JsValue>, key: &str, is_rquired: bool) -> Result<Option<u64>, AnyError> {
        if jsValue.is_a::<JsUndefined, _>(cx) && is_rquired {
          return Err(anyhow!("{} is required", key));
        } else if jsValue.is_a::<JsNumber, _>(cx) {
          match jsValue.to_string(cx).unwrap().value(cx).parse::<u64>() {
            Ok(res) => {
              return Ok(Option::from(res));
            },
            Err(err) => {
              return Err(anyhow!("{} type error123", key));
            },
          };
        }
      return Err(anyhow!("{} type error", key));
    }

    pub(crate) fn getOptionArray(mut cx: &mut FunctionContext, jsValue: Handle<JsValue>) -> Option<Vec<String>> {
      if jsValue.is_a::<JsArray, _>(cx) {
        let result = jsValue.to_string(cx).unwrap().value(cx);

        let result: Vec<String> = Regex::new(r",")
                .unwrap()
                .split(&result.to_owned())
                .map(|x| x.to_string())
                .collect();
        return Option::from(result);
      }
      return None;
    }

    pub(crate) fn getOptionAlgorithm(mut cx: &mut FunctionContext, jsValue: Handle<JsValue>) -> Option<Algorithm> {
      if jsValue.is_a::<JsString, _>(cx) {
        let result = jsValue.to_string(cx).unwrap().value(cx);
        let algorithm =  match result.as_str() {
          "HS256" => Algorithm::HS256,
          "HS384" => Algorithm::HS384,
          "HS512" => Algorithm::HS512,
          "ES256" => Algorithm::ES256,
          "ES384" => Algorithm::ES384,
          "RS256" => Algorithm::RS256,
          "RS384" => Algorithm::RS384,
          "RS512" => Algorithm::RS512,
          "PS256" => Algorithm::PS256,
          "PS384" => Algorithm::PS384,
          "PS512" => Algorithm::PS512,
          _ => Algorithm::HS256,
        };

        return Option::from(algorithm);
      }
      return None;
    }

    pub(crate) fn new(mut cx: &mut FunctionContext, chaimsObj: Handle<JsObject>) -> Result<Self, AnyError> {
      let secret = chaimsObj.get(cx, "secret").unwrap();
      let aud = chaimsObj.get(cx, "aud").unwrap();
      let exp = chaimsObj.get(cx, "exp").unwrap();
      let iat = chaimsObj.get(cx, "iat").unwrap();
      let iss = chaimsObj.get(cx, "iss").unwrap();
      let nbf = chaimsObj.get(cx, "nbf").unwrap();
      let sub = chaimsObj.get(cx, "sub").unwrap();
      let algorithm = chaimsObj.get(cx, "algorithm").unwrap();

      let mut chaims = Claims::default();
      chaims.secret = Claims::getOptionString(&mut cx, secret);
      chaims.aud = Claims::getOptionString(&mut cx, aud);

      chaims.iat = match Claims::getOptionNumber(&mut cx, iat, "iat", false) {
            Ok(val) => val,
            Err(err) => None,
          };
      chaims.nbf = match Claims::getOptionNumber(&mut cx, nbf, "nbf", false) {
        Ok(val) => val,
        Err(err) => None,
      };
      chaims.iss = Claims::getOptionString(&mut cx, iss);
      chaims.sub = Claims::getOptionString(&mut cx, sub);
      chaims.algorithm = Claims::getOptionAlgorithm(&mut cx, algorithm);
      match Claims::getOptionNumber(&mut cx, exp, "exp", true) {
        Ok(val) => {
          chaims.exp = val.unwrap();
        },
        Err(err) => {
          return Err(err);
        },
      };

     Ok(chaims)
    }

    /**
    * encode jwt
    */
    pub(crate) fn encode_jwt(&self) -> Result<String, AnyError> {
      let secret = match &self.secret {
        Some(secret) => secret,
        None => {
          return Err(anyhow!("secret is required"))
        }
      };

      let key = EncodingKey::from_secret(secret.as_ref());
      // Ok("shijinhua")
      match jsonwebtoken::encode(&Header::default(), &self, &key) {
        Ok(token) => Ok(token),
        Err(err) => {
          return Err(anyhow!("jwt encode error"));
        },
      }
    }

    /**
    * decode jwt
    */
    pub(crate) fn decode_jwt(token: &str, secret: String) -> Result<jsonwebtoken::TokenData<Claims>, AnyError> {
      let key = DecodingKey::from_secret(secret.as_ref());
      match jsonwebtoken::decode::<Claims>(token, &key, &Validation::default()) {
        Ok(claims) => Ok(claims),
        Err(err) => {
          return Err(anyhow!("jwt decode error"));
        },
      }
    }
  }

  pub struct JWT {
    // cx: ModuleContext<'a>
  }
  
  impl JWT {
      pub fn new(mut cx: ModuleContext<'_>) {
        cx.export_function("encode", Self::encode);
        cx.export_function("decode", Self::decode);
      }

    pub(crate) fn result<'a>(
      mut cx: &mut FunctionContext<'a>,
      error: bool,
      token: &str,
      message: &str,
      data: Option<Handle<JsObject>>,
    ) -> JsResult<'a, JsObject> {
      let result = cx.empty_object();
      let error = cx.boolean(error);
      let token = cx.string(token);
      let message = cx.string(message);

      result.set(cx, "error", error);
      result.set(cx,"token", token);
      result.set(cx,"message", message);
      match data {
        Some(data) => {
          result.set(cx,"data", data);
        },
        None => {},
      }


      Ok(result)
    }

    pub(crate) fn encode(mut cx: FunctionContext) -> JsResult<JsObject> {
      let chaimsObj = cx.argument::<JsObject>(0)?;

      match Claims::new(&mut cx, chaimsObj) {
        Ok(chaims) => {
           match chaims.encode_jwt() {
            Ok(token) => {
              JWT::result(&mut cx, false, token.as_str(), "成功", None )
            },
            Err(err) => {
              JWT::result(&mut cx, true, "",  &err.to_string().as_str(), None)
            }
          }
        },
        Err(err) => {
          JWT::result(&mut cx, true, "",  &err.to_string().as_str(), None)
        }
      }
    }
    pub(crate) fn decode(mut cx: FunctionContext) -> JsResult<JsObject> {
      let token = cx.argument::<JsValue>(0)?;
      let secret = cx.argument::<JsValue>(1)?;

      let token = Claims::getString(&mut cx, token);
      let secret = Claims::getString(&mut cx, secret);

      match Claims::decode_jwt(token.as_str(), secret) {
        Ok(claims) => {
          println!("{:?}", claims.claims);
          let data = claims.claims.toJsObject(&mut cx);
          JWT::result(&mut cx, false, "", "成功", data )
        },
        Err(err) => {
          JWT::result(&mut cx, true, "",  &err.to_string().as_str(), None)
        },
      }
    }
  }
}

