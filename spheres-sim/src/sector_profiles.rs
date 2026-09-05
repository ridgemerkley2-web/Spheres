//! Sourced broad national accounts with explicit within-sector game splits.
//! New-campaign presentation/accounting only: no extra GDP, money or inventory.
use crate::{districts,world::{NationId,WorldState}};
use serde::{Deserialize,Serialize};
use std::{collections::BTreeMap,sync::OnceLock};
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct Profile { pub shares:[f64;8], pub source:String, pub quality:String }
#[derive(Deserialize)]
struct Dataset {schema_version:u32,countries:BTreeMap<NationId,Profile>}
pub fn data()->&'static BTreeMap<NationId,Profile>{
    static DATA:OnceLock<BTreeMap<NationId,Profile>>=OnceLock::new();
    DATA.get_or_init(||{
        let data:Dataset=serde_json::from_str(include_str!("../data/sectors_1990.json")).expect("frozen sector profiles");
        assert_eq!(data.schema_version,1);
        for p in data.countries.values(){assert!(p.shares.iter().all(|v|v.is_finite()&&*v>=0.0));assert!((p.shares.iter().sum::<f64>()-1.0).abs()<1e-12);assert!(!p.source.is_empty());}
        data.countries
    })
}

/// Density is a limited urban-activity proxy, not measured provincial output.
/// A dense province receives at most 25% more mass per person; a sparse one at
/// most 25% less. The subsequent national normalization preserves exact GDP.
pub fn allocation_masses(w:&WorldState,ids:&[String],enriched:bool)->Vec<f64>{
    let pop:Vec<_>=ids.iter().map(|d|districts::population_of(w,d).unwrap_or(0.0).max(0.0)).collect();
    if !enriched{return pop;}
    let area:f64=ids.iter().map(|d|districts::area_of(d).max(0.0)).sum();
    let mean=pop.iter().sum::<f64>()/area.max(1.0);
    ids.iter().enumerate().map(|(i,d)|{
        let area=districts::area_of(d);
        if area<=0.0||pop[i]<=0.0||mean<=0.0 {return pop[i];}
        pop[i]*(1.0+0.1*((pop[i]/area)/mean).ln().clamp(-2.5,2.5))
    }).collect()
}
