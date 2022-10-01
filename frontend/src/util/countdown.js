export const timer = function (end, onTick, onComplete) {
  var timeLeft = end - new Date();

  var timeAPI = {
    DAYS: 1000 * 60 * 60 * 24,
    HOURS: 1000 * 60 * 60,
    MINUTES: 1000 * 60,
    SECONDS: 1000,
    MILL: 10,
  };

  var tick = function () {
    if (timeLeft > 0) {
      var time = timeLeft;
      var days = Math.floor(time / timeAPI.DAYS);
      time %= timeAPI.DAYS;
      var hours = Math.floor(time / timeAPI.HOURS);
      time %= timeAPI.HOURS;
      var minutes = Math.floor(time / timeAPI.MINUTES);
      time %= timeAPI.MINUTES;
      var seconds = Math.floor(time / timeAPI.SECONDS);
      time %= timeAPI.SECONDS;
      var mill = Math.floor(time / timeAPI.MILL);

      var countdown = {
        days,
        hours,
        minutes,
        seconds,
        mill,
      };
      onTick(countdown);
      timeLeft -= 10;
    } else {
      timeLeft -= 10;
      clearInterval(interval), onComplete();
    }
  };

  var interval = setInterval(
    (function (self) {
      return function () {
        tick.call(self);
      };
    })(this),
    10
  );

  var getTimeRemaining = function () {
    var time = timeLeft;
    var days = Math.floor(time / timeAPI.DAYS);
    time %= timeAPI.DAYS;
    var hours = Math.floor(time / timeAPI.HOURS);
    time %= timeAPI.HOURS;
    var minutes = Math.floor(time / timeAPI.MINUTES);
    time %= timeAPI.MINUTES;
    var seconds = Math.floor(time / timeAPI.SECONDS);
    time %= timeAPI.SECONDS;
    var mill = Math.floor(time / timeAPI.MILL);

    return {
      days,
      hours,
      minutes,
      seconds,
      mill,
    };
  };

  tick.call(this);

  return {
    abort: function () {
      clearInterval(interval);
    },
    getTimeRemaining: getTimeRemaining,
  };
};
