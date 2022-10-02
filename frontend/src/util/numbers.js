export const padZero = (number) => {
  if (number <= 99) {
    number = ("0" + number).slice(-2);
  }
  return number;
};
