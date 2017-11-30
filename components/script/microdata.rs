/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use serde_json;
use dom::node::{Node};
use dom::bindings::inheritance::{Castable};
use dom::element::{Element};
use dom::characterdata::{CharacterData};
use dom::text::Text;
use std::collections::HashMap;

pub struct Microdata {}
#[derive(Serialize,Clone)]
#[serde(untagged)]
enum Data {
    StrValue(String),
    VecValue(Vec<Data>),
    DataValue(Box<Data>),
    HashValue(HashMap<String,Data>)
}

impl Microdata {
   // [Pref="dom.microdata.testing.enabled"]    x
    pub fn parse( node: &Node) {
        let jsonData:Data = Self::traverse(node).unwrap();
      
        let json = serde_json::to_string(&jsonData);
        println!("printing json from microdata {:?}", json);
    }


  fn get_attr_value(element: &Element, property: &str)-> Option<String> {
    println!("{:?}",property);
    //   let mut atoms =  match property {
    //       "itemprop" => element.get_tokenlist_attribute(&local_name!("itemprop"), ),
    //       "itemtype" =>  element.get_tokenlist_attribute(&local_name!("itemtype"), ),
    //       _ => {},
    //   };
    //     if !atoms.is_empty() {
    //       let temp_key = atoms.remove(0);
    //       return Some(String::from(temp_key.trim()).to_owned());
    //     }
    //     else {
    //       return None;
    //     }
    Some(String::from("itemprop"))
    }
  
  fn traverse( node: &Node)-> Option<Data> { 
        if !node.is::<Element>(){
            if let Some(ref text) = node.downcast::<Text>(){
                let mut content = String::new();
                content.push_str(&text.upcast::<CharacterData>().data());
                return Some(Data::StrValue(String::from(content)));
            }
            None
        }
        else {
            let element = node.downcast::<Element>().unwrap();
            let mut headStr = String::from("");
            let mut parentVec:Vec<Data> = Vec::new();
            let itemType = Self::get_attr_value(element,"itemtype").unwrap();
            // if element.has_attribute(&local_name!("itemscope")) && element.has_attribute(&local_name!("itemtype")) && !element.has_attribute(&local_name!("itemprop")) {
            //     headStr = String::from("items");
            //     let mut propMap:HashMap<String,Data> = HashMap::new();
            //     //Data::HashValue(propMap)
            //     let itemType = Self::get_attr_value(element,"itemtype").unwrap();
                
            // }
            // else if element.has_attribute(&local_name!("itemprop")) && element.has_attribute(&local_name!("itemtype")) {
                
            //     headStr = Self::get_attr_value(element,"itemprop").unwrap();
            //     let itemType = Self::get_attr_value(element,"itemtype").unwrap();
            
            // }
            // else {
            //     return None;
            // }
            let mut innerMap:HashMap<String,Data> = HashMap::new();
            for child in node.children(){
                if let Some(childData) = Self::traverse(child.upcast::<Node>()){
                    parentVec.push(childData);
                }
            }
            innerMap.insert(headStr,Data::VecValue(parentVec));
            Some(Data::HashValue(innerMap))
        }
    }
}
