use serde::{Deserialize, Serialize};
use serde_json::to_string;
pub struct util{


}
impl util{

    pub fn to_string_or_default<T>(thing:T, default:String) -> String where T:for<'a> Deserialize<'a> + Serialize + Sized{
            let string = to_string(&thing);
            match string{
                Ok(x)=>x,
                _=>default
            }
    }


}