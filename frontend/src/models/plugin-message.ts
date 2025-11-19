export type FinishReason = "success" | "failure" | "timeout";

export interface PluginMessage {
  type: "finish";
  hasFinished: () => boolean;
  reason: FinishReason;
  statusCode: number;
}

export function parseEventData(eventData: any): PluginMessage | null {
  return {
    type: "finish",
    hasFinished: () => true,
    reason: parseReason(eventData.reason),
    statusCode: eventData.statusCode ?? 1,
  };
}

function parseReason(reason?: any): FinishReason {
  if (reason === "success") return "success";
  if (reason === "failure") return "failure";
  if (reason === "timeout") return "timeout";
  return "failure";
}
