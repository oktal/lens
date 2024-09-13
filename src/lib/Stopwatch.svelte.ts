import { Temporal } from '@js-temporal/polyfill';

class Stopwatch {
  currentTime: Temporal.PlainTime = Temporal.Now.plainTimeISO();
  startTime: Temporal.PlainTime | undefined = undefined;
  elapsed: Temporal.Duration | undefined = $state(undefined);

  totalPausedElapsed: Temporal.Duration | undefined = undefined;
  timerInterval: number | undefined = undefined;

  constructor() {
  }

  pause() {
    if (this.elapsed) {
      this.totalPausedElapsed = this.elapsed;
    }
    this.startTime = undefined;
  }

  resume() {
    this.startTime = Temporal.Now.plainTimeISO();
  }

  stop() {
    this.pause();

    this.timerInterval = undefined;
    if (this.timerInterval) clearInterval(this.timerInterval);
  }

  reset() {
    this.stop();
    this.elapsed = undefined;
  }

  restart() {
    this.totalPausedElapsed = undefined;
    this.start();
  }

  start() {
    this.startTime = Temporal.Now.plainTimeISO();

    this.timerInterval = setInterval(() => {
      this.currentTime = Temporal.Now.plainTimeISO();
      if (this.startTime) {
        const currentElapsed = this.currentTime.since(this.startTime);

        if (this.totalPausedElapsed) {
          this.elapsed = currentElapsed.add(this.totalPausedElapsed);
        } else {
          this.elapsed = currentElapsed;
        }
      }
    }, 10);
  }
}

export { Stopwatch }
