use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// GET /sched-api/activity/findByRunNameAndBeamlineId/{RunName}/{beamlineId}

#[derive(Serialize, Deserialize, Debug)]
struct Activity {
    activityId: Option<i64>,
    scheduleId: Option<i64>,
    activityName: Option<String>,
    startTime: Option<String>,
    endTime: Option<String>,
    duration: Option<i64>,
    utilization: Option<i64>,
    parentActivityId: Option<i64>,
    activityType: Option<ActivityType>,
    beamtime: Beamtime,
    experimentId: Option<i64>,
    station: Option<Station>,
    version: Option<i64>,
    activityComment: Option<String>,
    user: Option<UserType>,
    activityMessageConfig: Option<ActivityMessageConfig>,
    timeUnused: Option<i64>,
    displayColor: Option<i64>,
    clientFkId: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserType {
    badgeNo: Option<String>,
    firstName: Option<String>,
    lastName: Option<String>,
    name: Option<String>,
    userName: Option<String>,
    email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ActivityType {
    activityTypeId: Option<i64>,
    activityTypeName: Option<String>,
    activityTypeDescription: Option<String>,
    systemActivityFlag: Option<i64>,
    version: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug)]
struct preferredDate
{
        preferDtSeq: Option<i64>,
        gupId: Option<i64>,
        beamtimeId: Option<i64>,
        dateFrom: Option<String>,
        dateTo: Option<String>   
}

#[derive(Serialize, Deserialize, Debug)]
struct Beamtime {
    beamtimeId: Option<i64>,
    beamlineFirst: Option<Beamline>,
    beamlineSecond: Option<Beamline>,
    beamlineThird: Option<Beamline>,
    grantedBeamline: Option<Beamline>,
    scheduledBeamline1: Option<Beamline>,
    scheduledBeamline2: Option<Beamline>,
    scheduledBeamline3: Option<Beamline>,
    scheduledBeamline4: Option<Beamline>,
    proposal: Proposal,
    proposalStatus: ProposalStatus,
    schedulingPeriods: SchedulingPeriods,
    preferredDates: Vec<preferredDate>,
    requestedDate: Option<String>,
    actualShifts: Option<i64>,
    grantedShifts: Option<i64>,
    scheduledShifts: Option<i64>,
    scheduledShifts2: Option<i64>,
    scheduledShifts3: Option<i64>,
    scheduledShifts4: Option<i64>,
    equipment: Option<String>,
    rapidAccessFlag: Option<String>,
    anyBeamlineFlag: Option<String>,
    timeUnit: Option<i64>,
    declinedFlag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Beamline {
    beamlineNum: Option<i64>,
    beamlineId: Option<String>,
    beamlineIdOld: Option<String>,
    beamlineName: Option<String>,
    operator: Operator,
    source: Source,
    sector: Sector,
    inactiveDate: Option<String>,
    stations: Vec<Station>,
    supportedTechniques: Vec<SupportedTechnique>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Operator {
    operatorId: Option<i64>,
    operatorName: Option<String>,
    operatorShortName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Source {
    sourceId: Option<i64>,
    sourceName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sector {
    sectorId: Option<i64>,
    sectorName: Option<String>,
    sectorNum: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Station {
    stationId: Option<i64>,
    stationName: Option<String>,
    inactiveDate: Option<String>,
    createdDate: Option<String>,
    beamLineNum: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SupportedTechnique {
    supportedTechniquesId: SupportedTechniquesId,
    orderColumn: Option<i64>,
    collaborationOnlyFlag: Option<String>,
    technique: Technique,
}

#[derive(Serialize, Deserialize, Debug)]
struct SupportedTechniquesId {
    techniqueId: Option<i64>,
    beamLineNum: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Technique {
    techniqueId: Option<i64>,
    techniqueName: Option<String>,
    category: Option<String>,
    inactiveFlag: Option<String>,
    subCategory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Proposal {
    gupId: Option<i64>,
    proposalTitle: Option<String>,
    proprietaryFlag: Option<String>,
    pupId: Option<i64>,
    submittedDate: Option<String>,
    totalShiftsRequested: Option<i64>,
    mailInFlag: Option<String>,
    proposalStatus: Option<ProposalStatus>,
    proposalType: Option<ProposalType>,
    experimenters: Vec<Experimenter>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProposalStatus {
    statusId: Option<i64>,
    statusDesc: Option<String>,
    statusType: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProposalType {
    typeId: Option<String>,
    typeDescription: Option<String>,
    inactiveFlag: Option<String>,
    display: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Experimenter {
    gupExperimenterId: i64,
    badge: String,
    firstName: String,
    lastName: String,
    institution: String,
    email: Option<String>,
    piFlag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SchedulingPeriods {
    runStartDate: Option<String>,
    runEndDate: Option<String>,
    notifyUserDate: Option<String>,
    schedulingPeriods: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ActivityMessageConfig {
    activityMessageConfigId: Option<i64>,
    hold: Option<i64>,
    enableActivityScheduled: Option<i64>,
    activityScheduledStatus: Option<i64>,
    enableEsafReminder: Option<i64>,
    esafReminderStatus: Option<i64>,
    enableExpReminder: Option<i64>,
    expReminderStatus: Option<i64>,
    enablePubReminder: Option<i64>,
    pubReminderStatus: Option<i64>,
    customText: Option<String>,
    fromEmailAddr: Option<String>,
    version: Option<i64>,
    enableEndExpReminder: Option<i64>,
    expEndReminderStatus: Option<i64>,
}

pub fn parse_activity(json_data: &str, experimenter_lastname: &str) -> Result<(), serde_json::Error>
{
    let activities: Vec<Activity> = serde_json::from_str(json_data)?;
    activities.iter().for_each(|activity| 
    {
        //println!("{:?} {:?} {:?}", activity.activityId, activity.station, activity.experimentId);
        //println!{"{:?} {:?} {:?} {:?}", activity.beamtime.proposal.gupId, activity.beamtime.proposal.proposalTitle, activity.beamtime.proposal.experimenters, activity.beamtime.proposalStatus};
                    
        
        activity.beamtime.proposal.experimenters.iter().for_each(|experimenter| 
        {
            if experimenter.piFlag.is_some() && experimenter.lastName == experimenter_lastname
            {
                if experimenter.piFlag.is_some() && experimenter.piFlag.as_ref().unwrap() == "Y"
                {
                    //println!("{:?} {:?} {:?} {:?}", activity.activityId, activity.station, activity.experimentId, experimenter);
                    println!("{:?} {:?} {:?}", activity.activityId, activity.experimentId, experimenter);
                    //println!{"{:?}", activity.beamtime};
                    println!{"{:?} {:?} {:?}", activity.beamtime.proposal.gupId, activity.beamtime.proposal.proposalTitle, activity.beamtime.proposalStatus};
                    /*
                    if let Some(beamline_first) = &activity.beamtime.beamlineFirst 
                    {
                        println!("{:?} {:?} {:?}", beamline_first.source.sourceName, beamline_first.sector.sectorName, beamline_first.sector.sectorNum);
                    }
                    */
                }
            }
        });
        
    });
    
    //println!("{:?}", activity);

    Ok(())
}