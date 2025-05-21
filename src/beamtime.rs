use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// GET /sched-api/beamtimeRequests/findBeamtimeRequestsByRunAndBeamline/{schedulingPeriod}/{beamlineId}

#[derive(Serialize, Deserialize)]
struct Operator {
    operatorId: Option<i32>,
    operatorName: Option<String>,
    operatorShortName: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Source {
    sourceId: Option<i32>,
    sourceName: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Sector {
    sectorId: Option<i32>,
    sectorName: Option<String>,
    sectorNum: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct Station {
    stationId: Option<i32>,
    stationName: Option<String>,
    inactiveDate: Option<DateTime<Utc>>,
    createdDate: Option<DateTime<Utc>>,
    beamLineNum: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct Technique {
    techniqueId: Option<i32>,
    techniqueName: Option<String>,
    category: Option<String>,
    inactiveFlag: Option<String>,
    subCategory: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SupportedTechniquesId {
    techniqueId: Option<i32>,
    beamLineNum: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct SupportedTechniques {
    supportedTechniquesId: Option<SupportedTechniquesId>,
    orderColumn: Option<i32>,
    collaborationOnlyFlag: Option<String>,
    technique: Option<Technique>,
}

#[derive(Serialize, Deserialize)]
struct Beamline {
    beamlineNum: Option<i32>,
    beamlineId: Option<String>,
    beamlineIdOld: Option<String>,
    beamlineName: Option<String>,
    operator: Option<Operator>,
    source: Option<Source>,
    sector: Option<Sector>,
    getinactiveDate: Option<DateTime<Utc>>,
    stations: Option<Vec<Station>>,
    supportedTechniques: Option<Vec<SupportedTechniques>>,
}

#[derive(Serialize, Deserialize)]
struct ProposalStatus {
    getstatusId: Option<i32>,
    getstatusDesc: Option<String>,
    getstatusType: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ProposalType {
    typeId: Option<String>,
    typeDescription: Option<String>,
    inactiveFlag: Option<String>,
    display: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Experimenter {
    gupExperimenterId: Option<i32>,
    badge: Option<String>,
    firstName: Option<String>,
    lastName: Option<String>,
    institution: Option<String>,
    email: Option<String>,
    piFlag: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Proposal {
    gupId: Option<i32>,
    proposalTitle: Option<String>,
    proprietaryFlag: Option<String>,
    pupId: Option<i32>,
    submittedDate: Option<DateTime<Utc>>,
    totalShiftsRequested: Option<i32>,
    mailInFlag: Option<String>,
    proposalStatus: Option<ProposalStatus>,
    proposalType: Option<ProposalType>,
    experimenters: Option<Vec<Experimenter>>,
}

#[derive(Serialize, Deserialize)]
struct SchedulingPeriods {
    runStartDate: Option<DateTime<Utc>>,
    runEndDate: Option<DateTime<Utc>>,
    notifyUserDate: Option<DateTime<Utc>>,
    schedulingPeriods: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct PreferredDate {
    preferDtSeq: Option<i32>,
    gupId: Option<i32>,
    beamtimeId: Option<i32>,
    dateFrom: Option<DateTime<Utc>>,
    dateTo: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
struct Beamtime {
    beamtimeId: Option<i32>,
    beamlineFirst: Option<Beamline>,
    beamlineSecond: Option<Beamline>,
    beamlineThird: Option<Beamline>,
    grantedBeamline: Option<Beamline>,
    scheduledBeamline1: Option<Beamline>,
    scheduledBeamline2: Option<Beamline>,
    scheduledBeamline3: Option<Beamline>,
    scheduledBeamline4: Option<Beamline>,
    proposal: Option<Proposal>,
    proposalStatus: Option<ProposalStatus>,
    getschedulingPeriods: Option<SchedulingPeriods>,
    preferredDates: Option<Vec<PreferredDate>>,
    requestedDate: Option<DateTime<Utc>>,
    actualShifts: Option<i32>,
    grantedShifts: Option<i32>,
    scheduledShifts: Option<i32>,
    scheduledShifts2: Option<i32>,
    scheduledShifts3: Option<i32>,
    scheduledShifts4: Option<i32>,
    equipment: Option<String>,
    rapidAccessFlag: Option<String>,
    anyBeamlineFlag: Option<String>,
    timeUnit: Option<i32>,
    declinedFlag: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Run {
    runId: Option<i32>,
    runName: Option<String>,
    startTime: Option<DateTime<Utc>>,
    endTime: Option<DateTime<Utc>>,
    version: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct Data {
    beamtimeId: Option<i32>,
    schedulingPeriod: Option<String>,
    beamlineId: Option<String>,
    customGroup: Option<String>,
    piLastName: Option<String>,
    timeUnitString: Option<String>,
    requestedShifts: Option<i32>,
    grantedShifts: Option<i32>,
    beamlineScheduledShifts: Option<i32>,
    totalScheduledShifts: Option<i32>,
    beamlineRank: Option<String>,
    proposalTitle: Option<String>,
    piFirstName: Option<String>,
    piInstitution: Option<String>,
    typeDescription: Option<String>,
    localAccess: Option<String>,
    status: Option<String>,
    proposalType: Option<String>,
    loggedInBadgeNo: Option<String>,
    beamtime: Option<Beamtime>,
    beamline: Option<Beamline>,
    proposal: Option<Proposal>,
    run: Option<Run>,
    activiityTypeName: Option<String>,
}

pub fn parse_beamtime(json_data: &str, experimenter_lastname: &str) -> Result<(), serde_json::Error>
{
    let beamtime: Vec<Data> = serde_json::from_str(json_data)?;
    beamtime.iter().for_each(|activity: &Data| 
    {
        if activity.piLastName.is_some()
        {
            let pi_last_name = activity.piLastName.as_ref().unwrap();
            if experimenter_lastname == pi_last_name
            {   
                println!("{:?} {:?} {:?} {:?}", activity.piLastName, activity.activiityTypeName, activity.status, activity.proposalTitle);
            }
            else
            {
                println!("experimenter: {:?}", pi_last_name);
            }
        }
    });
    Ok(())
}
