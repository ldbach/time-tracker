// Format full date and time
export const formatDateTime = (isoString) => {
  if (!isoString) return "-";
  const date = new Date(isoString);
  return date.toLocaleString(); // local timezone
};

// Format only time
export const formatTime = (isoString) => {
  if (!isoString) return "-";
  return new Date(isoString).toLocaleTimeString();
};

// Format only date
export const formatDate = (isoString) => {
  if (!isoString) return "-";
  return new Date(isoString).toLocaleDateString();
};

// Format weekday name
export const formatWeekday = (isoString) => {
  if (!isoString) return "-";
  return new Date(isoString).toLocaleDateString(undefined, { weekday: "long" });
};