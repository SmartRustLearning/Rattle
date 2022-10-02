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
        <div class="task" v-bind:class="{ pulse: shouldPulseTask }">
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
        <!-- Computing Results Modal -->
        <div v-if="showComputeResultsModal" class="modal">
          <h2 style="margin-top: 80px; font-size: 2.4rem">
            Computing results...
          </h2>
          <div style="display: flex">
            <img src="../assets/Crab-unscreen.gif" alt="" class="modalImage" />
          </div>
        </div>
        <!-- Winner -->
        <div v-else-if="isWinner" class="modal">
          <div
            style="display: flex; justify-content: space-evenly; padding: 10px"
          >
            <h2 class="modal-title">Congrats! You won! 🎉🤓</h2>
            <div style="display: flex; justify-content: center">
              <img class="loserPic" :src="player.picture" />
            </div>
          </div>
          <div
            style="width: 100%; display: flex; justify-content: space-around"
          >
            <router-link to="/" style="margin: auto">
              <button class="button">exit 🚮</button>
            </router-link>
            <button class="button" style="margin: auto" @click="nextGame">
              next ➡️
            </button>
          </div>
        </div>
        <!-- Loser -->
        <div v-else class="modal">
          <div
            style="display: flex; justify-content: space-evenly; padding: 10px"
          >
            <h2 class="modal-title">You weren't successful this time... 😢</h2>
            <div style="display: flex; justify-content: center">
              <img class="loserPic" :src="player.pictureLoser" />
            </div>
          </div>

          <div style="display: flex; flex-direction: column; padding: 10px">
            <h3 style="margin: 0; text-align: left">Explanation</h3>
            <p style="display: flex; text-align: left">
              {{ explanation }}
            </p>
          </div>
          <div
            style="width: 100%; display: flex; justify-content: space-around"
          >
            <router-link to="/" style="margin: auto">
              <button class="button">exit 🚮</button>
            </router-link>
            <button class="button" style="margin: auto" @click="nextGame">
              next ➡️
            </button>
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<script>
import Vue from "vue";
import VueConfetti from "vue-confetti";

import CodeEditor from "simple-code-editor";
import { exercises } from "../data/exercises";
import { timer } from "../util/countdown";
import { padZero } from "../util/numbers";
import axios from "axios";

Vue.use(VueConfetti);

const socket = new WebSocket("ws://127.0.0.1:5000/websocket");

socket.onopen = function () {
  console.log("websocket:");
  console.info("[open] Connection established");
  console.info("Sending to server");
  socket.send(this.value);
  console.info("🦀 🦀 Compiling...");
};

socket.onmessage = function (event) {
  console.info(`[message] Data received from server: ${event.data}`);
};

import { players } from "../data/players";
import router from "@/router";

function handleSave() {
  const timeLeft = this.countdown.getTimeRemaining();
  console.log("handle save", this.value, {
    timeLeft,
    date: new Date(),
  });

  this.openModal();
  this.toggleComputeResultsModal();
  this.countdown?.abort();
  setTimeout(() => {
    if (this.isWinner) {
      this.startConfetti();
      setTimeout(() => {
        this.stopConfetti();
      }, 5000);
    }
    this.toggleComputeResultsModal();
  }, 3000);
}

export default {
  data() {
    return {
      exerciseId: 0,
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
      showComputeResultsModal: false,
      opponent: players[1],
      player: players[0],
      matchID: "",
      shouldPulseTask: true,
      isWinner: false,
      isComputingResults: true,
    };
  },
  mounted() {
    // players + winner / loser
    if (localStorage.getItem("isSecondGame") === 1) {
      this.isWinner = localStorage.getItem("isWinnerOfGame2");
      this.player = players[localStorage.getItem("playerIdxRound2")];
      this.opponent = players[localStorage.getItem("opponentIdxRound2")];

      // // find opponent randomly
      // const randomOpponent =
      //   players[Math.floor(0 + Math.random() * (3 - 0 + 1))];
      // this.opponent = {
      //   name: randomOpponent.name,
      //   picture: randomOpponent.picture,
      // };
    } else {
      this.isWinner = localStorage.getItem("isWinnerOfGame1");
      this.player = players[localStorage.getItem("playerIdxRound1")];
      this.opponent = players[localStorage.getItem("opponentIdxRound1")];
    }

    // player
    this.player = players[localStorage.getItem("playerIdx")];

    // exercise
    this.exerciseId = localStorage.getItem("exerciseId");
    const duration = exercises[this.exerciseId].duration;

    this.value = exercises[this.exerciseId ?? 0].starterCode;
    this.problem = exercises[this.exerciseId ?? 0].problem;
    this.explanation = exercises[this.exerciseId ?? 0].explanation;

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
        this.toggleComputeResultsModal();
        this.countdown?.abort();
        setTimeout(() => {
          if (this.isWinner) {
            this.startConfetti();
            setTimeout(() => {
              this.stopConfetti();
            }, 4000);
          }
        }, 3000);
      }
    );

    // Pulse task for first 5 seconds
    setTimeout(() => {
      this.shouldPulseTask = false;
    }, 5000);
  },
  destroyed() {
    this.countdown?.abort();
  },
  methods: {
    startConfetti() {
      this.$confetti.start();
    },
    stopConfetti() {
      this.$confetti.stop();
    },
    handleSave,
    openModal() {
      this.modal = !this.modal;
    },
    toggleComputeResultsModal() {
      this.showComputeResultsModal = !this.showComputeResultsModal;
    },
    nextGame() {
      localStorage.setItem("isSecondGame", true);
      router.push("/");
    },
  },
  components: {
    CodeEditor,
  },
  created() {
    //Request to get the matchID, so we can implement that into the websocket link
    //Axios and Fetch!
    console.log("vue created");
    const headers = { "Content-Type": "application/json" };
    this.matchID = axios
      .get("http://10.0.4.138:5000/match/find", { header: headers })
      .then((response) => console.log(response));
    /*  this.matchID = await fetch('http://10.0.4.138:5000/match/find', {headers}); */
    console.log("matchID ==> ", this.matchID);
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

.loserPic {
  height: 120px;
  width: 120px;
  border-radius: 12px;
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

.btn .button {
  width: 150px;
  margin-top: 50px;
}

.button {
  width: 150px;
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
  z-index: 1000;
}

.modal {
  width: 600px;
  height: 400px;
  border-radius: 12px;
  background: white;
}

.modal-title {
  font-weight: bold;
  font-size: 24pt;
}

.pulse {
  animation: pulse-animation 1.5s infinite;
}

@keyframes pulse-animation {
  0% {
    box-shadow: 0 0 0 0px rgba(0, 0, 0, 0.2);
  }
  100% {
    box-shadow: 0 0 0 20px rgba(0, 0, 0, 0);
  }
}

.modalImage {
  width: 500px;
  position: relative;
  top: -75px;
  right: -120px;
}
</style>
