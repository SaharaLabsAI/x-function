import _ from "lodash";

export interface ServiceDetail {
  phase: string;
  status: string;
}

export interface FormattedDetail {
  phase: string;
  status: string;
}

const PHASE_NAMES = [
  "Fetching function information...",
  "Building function image...",
  "Running function...",
] as const;

export function formatServiceDetails(
  details: ServiceDetail[] = []
): FormattedDetail[] {
  if (_.isEmpty(details)) return [];

  return _.compact([
    details[0] && {
      phase: PHASE_NAMES[0],
      status: details[0].status,
    },

    details[1] && {
      phase: PHASE_NAMES[1],
      status: details[1].status,
    },

    details.length >= 3 && {
      phase: PHASE_NAMES[2],
      status: getRunningStatus(details.slice(2)),
    },
  ]);
}

function getRunningStatus(details: ServiceDetail[]): string {
  const hasFalse = _.some(details, { status: "False" });
  if (hasFalse) {
    return "False";
  }

  const allTrue = _.every(details, { status: "True" });
  if (allTrue) {
    return "True";
  }

  return "Unknown";
}