<template>
  <main>
    <div class="infoBar">
      <div class="profile">
        <img class="profilePic" :src="player.picture" />
        <p>{{ player.name }}</p>
      </div>
      <div class="timer">
        <p>
          {{ time.minutes }}:{{ time.seconds }}.<span
            style="font-size: 1.6rem"
            >{{ time.millis }}</span
          >
        </p>
      </div>
      <div class="profile">
        <img class="profilePic" :src="opponent.picture" />
        <p>{{ opponent.name }}</p>
      </div>
    </div>
    <div class="content">
      <div class="left">
        <code-editor
          v-model="value"
          :hide_header="true"
          :languages="[['rust', 'Rust']]"
          width="100%"
          height="100%"
          class="github_dark"
        ></code-editor>
      </div>
      <div class="right">
        <h2 class="task-title">Your Task</h2>
        <div class="task">
          <div class="task-text">{{ problem }}</div>
        </div>
      </div>
    </div>
    <div class="btn">
      <button class="button" @click="handleSave()">submit</button>
    </div>
    <div
      @click="modal = false"
      v-if="modal"
      style="width: 100vw; height: 100vh"
    >
      <div class="modalBackground" @click.stop>
        <div class="modal">
          <h2 style="margin-top: 80px; font-size: 2.4rem">
            {{ explanation }}
          </h2>
          <div style="display: flex">
            <img src="../assets/Crab-unscreen.gif" alt="" />
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<script>
import CodeEditor from "simple-code-editor";
import { exercises } from "../data/exercises";
import { timer } from "../util/countdown";
import { padZero } from "../util/numbers";

const socket = new WebSocket("ws://127.0.0.1:5000/websocket");

socket.onopen = function () {
  console.info("[open] Connection established");
  console.info("Sending to server");
  socket.send("This is some code...");
};

socket.onmessage = function (event) {
  console.info(`[message] Data received from server: ${event.data}`);
};

import { player, opponents } from "../data/players";

const duration = exercises[0].duration;

function handleSave() {
  const timeLeft = this.countdown.getTimeRemaining();
  console.log("handle save", this.value, {
    timeLeft,
    date: new Date(),
  });
  this.openModal();
  this.countdown?.abort();
}

export default {
  data() {
    return {
      value: exercises[0].starterCode,
      problem: exercises[0].problem,
      explanation: exercises[0].explanation,
      time: {
        millis: "00",
        seconds: "00",
        minutes: "00",
      },
      countdown: undefined,
      modal: false,
      opponent: {
        name: "???",
        picture: undefined,
      },
      player: {
        name: player.name,
        picture: player.picture,
      },
    };
  },
  mounted() {
    // TODO: Request match data from backend
    // Send start time!

    // find opponent randomly
    const randomOpponent =
      opponents[Math.floor(0 + Math.random() * (4 - 0 + 1))];
    this.opponent = {
      name: randomOpponent.name,
      picture: randomOpponent.picture,
    };

    const endTime = new Date(new Date().getTime() + 1000 * duration);
    this.countdown = timer(
      endTime,
      (timeLeft) => {
        this.time.seconds = padZero(timeLeft.seconds);
        this.time.minutes = padZero(timeLeft.minutes);
        this.time.millis = padZero(timeLeft.mill);
      },
      () => {
        console.log("time over -> open modal to solution screen", {
          date: new Date(),
        });
        this.openModal();
        this.countdown?.abort();
      }
    );
  },
  destroyed() {
    this.countdown?.abort();
  },
  methods: {
    handleSave,
    openModal() {
      this.modal = !this.modal;
    },
  },
  components: {
    CodeEditor,
  },
};
</script>

<style scoped>
main {
  display: block;
  width: 100%;
  height: 100%;
  margin: 0;
  margin-top: 30px;
}

.infoBar {
  display: flex;
  justify-content: space-between;
}

.profile {
  display: flex;
  font-size: 2rem;
  font-weight: bold;
}

.profile p {
  margin: auto;
  padding-left: 10px;
}

.profilePic {
  height: 80px;
  width: 80px;
  margin: auto;
  background-color: #bbb;
  border-radius: 50%;
  display: inline-block;
  box-shadow: 0 0 50px #ccc;
}

.timer {
  background: #e43717;
  display: inline-block;
  border-radius: 4px;
  padding: 10px 20px;
  width: 150px;
  box-shadow: 0 0 50px #ccc;
}

.timer p {
  text-align: center;
  color: white;
  font-size: 2.6rem;
  margin: 0;
  font-weight: bold;
}

.content {
  height: 550px;
  width: 100%;
  display: flex;
  justify-content: space-between;
  padding-top: 50px;
}

.left {
  height: 100%;
  width: 100%;
  margin: 0;
}

.right {
  display: flex;
  flex-direction: column;
  width: 400px;
  margin-left: 20px;
}

.task {
  padding: 15px;
  border-radius: 12px;
  background: #686868;
  box-shadow: 0 0 50px #ccc;
}

.task-title {
  padding: 0;
  padding-bottom: 10pt;
  margin: 0;
  font-size: 24pt;
  line-height: 28pt;
}

.task-text {
  font-size: 15pt;
  line-height: 20pt;
  color: white;
}

.editor {
  height: 100%;
  border-radius: 4px;
  background: rgb(200, 200, 200);
  margin: 0;
}

.btn {
  display: flex;
  justify-content: flex-start;
}

.button {
  width: 150px;
  margin-top: 50px;
}

.modalBackground {
  background: #00000079;
  position: absolute;
  display: flex;
  width: 100vw;
  height: 100vh;
  top: 0;
  left: 0;
  justify-content: center;
  align-items: center;
  text-align: center;
}

.modal {
  width: 600px;
  height: 400px;
  border-radius: 12px;
  background: white;
}
</style>
