use std::collections::HashMap;
use std::hash::Hash;
use std::process::{Termination, ExitCode};
use std::path::Path;
use std::env;

pub const IMT_SERVICES_DIR: &str = "IMT_SERVICES_DIR";

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Directory {
    Frontend,
    Backend
}
pub enum ProgramStatus {
    SUCCESS,
    FAILED
}

impl Termination for ProgramStatus {
    fn report(self) -> ExitCode {
        match self {
            ProgramStatus::SUCCESS => ExitCode::from(0),
            ProgramStatus::FAILED => ExitCode::from(1)
        }
    }
}
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum Nub {
    APIRouter,
    Behandle,
    Billing,
    BusinessRules,
    CMS,
    Filestore,
    Forms,
    Mapping,
    Paglipat,
    PlatformAdmin,
    PolicyData,
    ServiceTracker,
    Users,
    Vinna,
}

impl Nub {
    pub fn from_string(s: &str) -> Option<Self> {
        let s = s.to_lowercase();
        match s.as_str() {
            "apirouter" | "api" => Some(Nub::APIRouter),
            "billing" => Some(Nub::Billing),
            "behandle" | "bh" => Some(Nub::Behandle),
            "business-rules" | "br" => Some(Nub::BusinessRules),
            "cms" => Some(Nub::CMS),
            "filestore" | "fs" => Some(Nub::Filestore),
            "forms" => Some(Nub::Forms),
            "mapping" => Some(Nub::Mapping),
            "paglipat"=> Some(Nub::Paglipat),
            "platform-admin" | "pa" => Some(Nub::PlatformAdmin),
            "policy-data" | "pd" => Some(Nub::PolicyData),
            "service-tracker" | "st" => Some(Nub::ServiceTracker),
            "users" => Some(Nub::Users),
            "vinna" => Some(Nub::Vinna),
            _ => None,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Nub::APIRouter => "api".to_string(),
            Nub::Behandle => "behandle".to_string(),
            Nub::Billing => "billing".to_string(),
            Nub::BusinessRules => "business-rules".to_string(),
            Nub::CMS => "cms".to_string(),
            Nub::Filestore => "filestore".to_string(),
            Nub::Forms => "forms".to_string(),
            Nub::Mapping => "mapping".to_string(),
            Nub::Paglipat => "paglipat".to_string(),
            Nub::PlatformAdmin => "platform-admin".to_string(),
            Nub::PolicyData => "policy-data".to_string(),
            Nub::ServiceTracker => "service-tracker".to_string(),
            Nub::Users => "users".to_string(),
            Nub::Vinna => "vinna".to_string(),
        }
    }

    pub fn as_path_str(&self) -> String {
        String::from(env::var(IMT_SERVICES_DIR).unwrap() + "/" + &self.as_string()) // todo need to unwrap imt_services_dir cleaner
    }

    pub fn as_local_frontend_url(&self) -> String {
        let mut mappings = HashMap::new();
        mappings.insert(Nub::BusinessRules, "VUE_APP_ADMIN_URL_BUSINESS_RULES=https://localhost.imtins.com:8013".to_string());
        mappings.insert(Nub::CMS, "VUE_APP_ADMIN_URL_CMS=https://localhost.imtins.com:8015".to_string());
        mappings.insert(Nub::Filestore, "VUE_APP_ADMIN_URL_FILESTORE=https://localhost.imtins.com:8019".to_string());
        mappings.insert(Nub::Forms, "VUE_APP_ADMIN_URL_FORMS=https://localhost.imtins.com:8014".to_string());
        mappings.insert(Nub::Users, "VUE_APP_ADMIN_URL_USERS=https://localhost.imtins.com:8018".to_string());
        mappings.insert(Nub::Mapping, "VUE_APP_ADMIN_URL_MAPPING=https://localhost.imtins.com:8010".to_string());
        mappings.insert(Nub::PolicyData, "VUE_APP_POLICY_DATA_URL=https://localhost.imtins.com:8012".to_string());
        mappings.insert(Nub::PlatformAdmin, "VUE_APP_ADMIN_URL_PLATFORM_ADMIN=https://localhost.imtins.com:8016".to_string());
        mappings.insert(Nub::Vinna, "VUE_APP_ADMIN_URL_VINNA=https://localhost.imtins.com:8009".to_string());
        mappings.insert(Nub::Behandle, "VUE_APP_BEHANDLE_URL=https://localhost.imtins.com:8020".to_string());
        mappings.insert(Nub::Billing, "VUE_APP_BILLING_URL=https://localhost.imtins.com:8022".to_string());

        String::from(mappings.get(self).unwrap())
    }

    pub fn as_local_backend_url(&self) -> String {
        let mut mappings = HashMap::new();
        mappings.insert(Nub::BusinessRules, "INTERNAL_NUBS['business_rules']=\"https://br:7004\"".to_string());
        mappings.insert(Nub::CMS, "INTERNAL_NUBS['cms']=\"https://cms:7001\"".to_string());
        mappings.insert(Nub::Filestore, "INTERNAL_NUBS['filestore']=\"https://fs:7005\"".to_string());
        mappings.insert(Nub::Forms, "INTERNAL_NUBS['forms']=\"https://forms:7002\"".to_string());
        mappings.insert(Nub::Users, "INTERNAL_NUBS['users']=\"https://users:7007\"".to_string());
        mappings.insert(Nub::Mapping, "INTERNAL_NUBS['mapping']=\"https://mapping:7015\"".to_string());
        mappings.insert(Nub::PolicyData, "INTERNAL_NUBS['policy_data']=\"https://pd:7003\"".to_string());
        mappings.insert(Nub::PlatformAdmin, "INTERNAL_NUBS['platform_admin']=\"https://pa:7000\"".to_string());
        mappings.insert(Nub::Vinna, "INTERNAL_NUBS['vinna']=\"https://vinna:7016\"".to_string());
        mappings.insert(Nub::Behandle, "INTERNAL_NUBS['behandle']=\"https://behandle:7010\"".to_string());
        mappings.insert(Nub::Billing, "INTERNAL_NUBS['billing']=\"https://billing:7009\"".to_string());
        mappings.insert(Nub::ServiceTracker, "INTERNAL_NUBS['service_tracker']=\"https://st:7006\"".to_string());
        mappings.insert(Nub::APIRouter, "INTERNAL_NUBS['api_router']=\"https://api:7013\"".to_string());
        mappings.insert(Nub::Paglipat, "INTERNAL_NUBS['paglipat']=\"https://paglipat:7017\"".to_string());

        mappings.get(self).unwrap().clone()
    }

    pub fn set_as_wd(&self, dir_type: &Directory) {
        if *dir_type == Directory::Frontend {
            set_working_dir(&format!("{}{}", self.as_path_str(), "/frontend"));
        }
        else if *dir_type == Directory::Backend {
            set_working_dir(&format!("{}{}", self.as_path_str(), "/project/settings"));
        }
    }
}

pub fn set_working_dir(str_path: &String) -> bool {
    let wd = Path::new(str_path);
    let res = env::set_current_dir(&wd);
    res.is_ok()
}