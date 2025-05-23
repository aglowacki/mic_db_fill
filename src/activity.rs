use serde::{Deserialize, Serialize};

// GET /sched-api/activity/findByRunNameAndBeamlineId/{RunName}/{beamlineId}

#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    pub activityId: Option<i64>,
    pub scheduleId: Option<i64>,
    pub activityName: Option<String>,
    pub startTime: Option<String>,
    pub endTime: Option<String>,
    pub duration: Option<i64>,
    pub utilization: Option<i64>,
    pub parentActivityId: Option<i64>,
    pub activityType: Option<ActivityType>,
    pub beamtime: Beamtime,
    pub experimentId: Option<i64>,
    pub station: Option<Station>,
    pub version: Option<i64>,
    pub activityComment: Option<String>,
    pub user: Option<UserType>,
    pub activityMessageConfig: Option<ActivityMessageConfig>,
    pub timeUnused: Option<i64>,
    pub displayColor: Option<i64>,
    pub clientFkId: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserType {
    pub badgeNo: Option<String>,
    pub firstName: Option<String>,
    pub lastName: Option<String>,
    pub name: Option<String>,
    pub userName: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityType {
    pub activityTypeId: Option<i64>,
    pub activityTypeName: Option<String>,
    pub activityTypeDescription: Option<String>,
    pub systemActivityFlag: Option<i64>,
    pub version: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct preferredDate
{
    pub preferDtSeq: Option<i64>,
    pub gupId: Option<i64>,
    pub beamtimeId: Option<i64>,
    pub dateFrom: Option<String>,
    pub dateTo: Option<String>   
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Beamtime {
    pub beamtimeId: Option<i64>,
    pub beamlineFirst: Option<Beamline>,
    pub beamlineSecond: Option<Beamline>,
    pub beamlineThird: Option<Beamline>,
    pub grantedBeamline: Option<Beamline>,
    pub scheduledBeamline1: Option<Beamline>,
    pub scheduledBeamline2: Option<Beamline>,
    pub scheduledBeamline3: Option<Beamline>,
    pub scheduledBeamline4: Option<Beamline>,
    pub proposal: Proposal,
    pub proposalStatus: ProposalStatus,
    pub schedulingPeriods: SchedulingPeriods,
    pub preferredDates: Vec<preferredDate>,
    pub requestedDate: Option<String>,
    pub actualShifts: Option<i64>,
    pub grantedShifts: Option<i64>,
    pub scheduledShifts: Option<i64>,
    pub scheduledShifts2: Option<i64>,
    pub scheduledShifts3: Option<i64>,
    pub scheduledShifts4: Option<i64>,
    pub equipment: Option<String>,
    pub rapidAccessFlag: Option<String>,
    pub anyBeamlineFlag: Option<String>,
    pub timeUnit: Option<i64>,
    pub declinedFlag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Beamline {
    pub beamlineNum: Option<i64>,
    pub beamlineId: Option<String>,
    pub beamlineIdOld: Option<String>,
    pub beamlineName: Option<String>,
    pub operator: Operator,
    pub source: Source,
    pub sector: Sector,
    pub inactiveDate: Option<String>,
    pub stations: Vec<Station>,
    pub supportedTechniques: Vec<SupportedTechnique>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operator {
    pub operatorId: Option<i64>,
    pub operatorName: Option<String>,
    pub operatorShortName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub sourceId: Option<i64>,
    pub sourceName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sector {
    pub sectorId: Option<i64>,
    pub sectorName: Option<String>,
    pub sectorNum: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Station {
    pub stationId: Option<i64>,
    pub stationName: Option<String>,
    pub inactiveDate: Option<String>,
    pub createdDate: Option<String>,
    pub beamLineNum: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupportedTechnique {
    pub supportedTechniquesId: SupportedTechniquesId,
    pub orderColumn: Option<i64>,
    pub collaborationOnlyFlag: Option<String>,
    pub technique: Technique,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupportedTechniquesId {
    pub techniqueId: Option<i64>,
    pub beamLineNum: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Technique {
    pub techniqueId: Option<i64>,
    pub techniqueName: Option<String>,
    pub category: Option<String>,
    pub inactiveFlag: Option<String>,
    pub subCategory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proposal {
    pub gupId: Option<i32>,
    pub proposalTitle: Option<String>,
    pub proprietaryFlag: Option<String>,
    pub pupId: Option<i64>,
    pub submittedDate: Option<String>,
    pub totalShiftsRequested: Option<i64>,
    pub mailInFlag: Option<String>,
    pub proposalStatus: Option<ProposalStatus>,
    pub proposalType: Option<ProposalType>,
    pub experimenters: Vec<Experimenter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalStatus {
    pub statusId: Option<i64>,
    pub statusDesc: Option<String>,
    pub statusType: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalType {
    pub typeId: Option<String>,
    pub typeDescription: Option<String>,
    pub inactiveFlag: Option<String>,
    pub display: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Experimenter {
    pub gupExperimenterId: i64,
    pub badge: String,
    pub firstName: String,
    pub lastName: String,
    pub institution: String,
    pub email: Option<String>,
    pub piFlag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchedulingPeriods {
    pub runStartDate: Option<String>,
    pub runEndDate: Option<String>,
    pub notifyUserDate: Option<String>,
    pub schedulingPeriods: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityMessageConfig {
    pub activityMessageConfigId: Option<i64>,
    pub hold: Option<i64>,
    pub enableActivityScheduled: Option<i64>,
    pub activityScheduledStatus: Option<i64>,
    pub enableEsafReminder: Option<i64>,
    pub esafReminderStatus: Option<i64>,
    pub enableExpReminder: Option<i64>,
    pub expReminderStatus: Option<i64>,
    pub enablePubReminder: Option<i64>,
    pub pubReminderStatus: Option<i64>,
    pub customText: Option<String>,
    pub fromEmailAddr: Option<String>,
    pub version: Option<i64>,
    pub enableEndExpReminder: Option<i64>,
    pub expEndReminderStatus: Option<i64>,
}

pub fn search_for_pi_activity<'a>(activities: &'a Vec<Activity>, experimenter_lastname: &str) -> (Option<&'a Activity>, Option<&'a Experimenter>)
{
    let mut found_act = None;
    let mut found_exp = None;
    activities.iter().for_each(|activity| 
    {
        activity.beamtime.proposal.experimenters.iter().for_each(|experimenter: &Experimenter| 
        {
            if experimenter.piFlag.is_some() && experimenter.lastName == experimenter_lastname
            {
                if experimenter.piFlag.is_some() && experimenter.piFlag.as_ref().unwrap() == "Y"
                {
                    //println!("found pi: {} {}", experimenter.firstName, experimenter.lastName);
                    found_act = Some(activity);
                    found_exp = Some(experimenter);
                }
            }
        });
    });
    (found_act, found_exp)
}