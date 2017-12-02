* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use serde_json;
use dom::node::{Node};
use dom::bindings::inheritance::{Castable};
use dom::element::{Element};
use dom::characterdata::{CharacterData};
use dom::text::Text;
use std::collections::HashMap;
use servo_atoms::Atom;
use std::borrow::Cow;

pub struct Microdata {}
#[derive(Serialize,Clone)]
#[serde(untagged)]
// this is a recursive data structure for the json representation
enum Data {
    VecString(Vec<String>),
    VecValue(Vec<Data>),
    DataValue{
    // since 'type' is a reserved word
    #[serde(rename = "type")]   typ:Vec<String>,
        properties:HashMap<String,Data>
    },
    HashValue(HashMap<String,Data>),
}
//serde will insert the text 'type' and 'properties' automatically.

impl Microdata {
    pub fn parse( node: &Node) {
        let jsonData:Data = Self::traverse(node).unwrap();
      
        let json = serde_json::to_string(&jsonData);
        println!("printing json from microdata {:?}", json);
    }


  fn get_attr_value(atoms: &mut Vec<Atom>)-> Option<String> {
        if !atoms.is_empty() {
          let temp_key = atoms.remove(0);
          return Some(String::from(temp_key.trim()).to_owned());
        }
        else {
          return None;
        }
    }
  //traverse the DOM recursively by DFS
    fn traverse( node: &Node)-> Option<Data> { 
    // when we have reached the deepest Node i.e a CharacterData Node, return a Vec<String> 
    // This check is now being made where the child nodes are iterated
        // if !node.is::<Element>(){
        //     if let Some(ref text) = node.downcast::<Text>(){
        //         let mut content = String::new();
        //         content.push_str(&text.upcast::<CharacterData>().data());
        //         let mut propValVec:Vec<String> = Vec::new();
        //         propValVec.push(String::from(content));
        //         return Some(Data::VecString(propValVec));
        //     }
        //     None
        // }
        if let Some(element) = node.downcast::<Element>(){
                
            let mut key:String = String::from("");
            let mut isTopLevelItem:bool = false;

            //look for a top level tag with itemscope
            if element.has_attribute(&local_name!("itemscope")) && element.has_attribute(&local_name!("itemtype")) && !element.has_attribute(&local_name!("itemprop")) {
                 isTopLevelItem = true;
            }
           
            //instantiate a property map
            let mut propertyMap:HashMap<String,Data> = HashMap::new();
            //get child node information to store in the property map
            for child in node.children(){
                // the below code needs some modifications to handle returned information correctly.
                
                
                if let Some(candidate_node) = child.GetFirstChild() {
                    if let Some(ref text) = candidate_node.downcast::<Text>(){
                        let mut content = String::new();
                        content.push_str(&text.upcast::<CharacterData>().data());
                        let mut propValVec:Vec<String> = Vec::new();
                        propValVec.push(String::from(content));
                        return Some(Data::VecString(propValVec));
                    }
                }
                if let Some(childElement) = child.upcast::<Node>().downcast::<Element>(){
                    if childElement.has_attribute(&local_name!("itemprop")) && !childElement.has_attribute(&local_name!("itemtype")) {
                        key = Self::get_attr_value(&mut childElement.get_tokenlist_attribute(&local_name!("itemprop"),)).unwrap();
                    }
                }
                
                // recurse
                if let Some(childData) = Self::traverse(child.upcast::<Node>()){
                // needs modification to get the key safely into the map
                if let Data::VecString(entry) = childData{
                        propertyMap.insert(Cow::Borrowed(&key).to_string(), Data::VecString(entry));
                    }
                    else if let Data::VecValue(entry) = childData{
                        propertyMap.insert(Cow::Borrowed(&key).to_string(),Data::VecValue(entry));
                    }
                    // if a top level element is encountered return it, although, hacky.
                    else if let Data::HashValue(entry) = childData{
                        if entry.contains_key("items"){
                            return Some(Data::HashValue(entry));
                        }
                    }   
                }
            }   
            // if top level item is encountered, create the outer structure of the JSON
            // use the property map created earlier as the value, for the key - "properties"
            if isTopLevelItem {
                let headStr = String::from("items");
                let mut itemTypeVec:Vec<String> = Vec::new();
                let itemType = Self::get_attr_value(&mut element.get_tokenlist_attribute(&local_name!("itemtype"), )).unwrap();
                itemTypeVec.push(itemType);
                let mut outerVec:Vec<Data> = Vec::new();
                let mut outerMap:HashMap<String,Data> = HashMap::new();
                outerVec.push(Data::DataValue{typ:itemTypeVec,properties: propertyMap});
                outerMap.insert(headStr,  Data::VecValue(outerVec.to_vec()) );
                return Some(Data::HashValue(outerMap));
            }

           return None;
        }
        else {
            None;
        }    
         
    }
    
    
}
