<template>
  <main>
    <div class="infoBar">
      <div class="profile">
        <div class="profilePic"></div>
        <p>Player 1</p>
      </div>
      <div class="timer">
        <p>00:00.<span style="font-size: 1.6rem">00</span></p>
      </div>
      <div class="profile">
        <div class="profilePic"></div>
        <p>Player 2</p>
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
  </main>
</template>

<script>
import CodeEditor from "simple-code-editor";
import { exercises } from "../data/exercises";

function handleSave() {
  console.log("handle save", this.value);
}

export default {
  data() {
    return {
      value: exercises[0].starterCode,
      problem: exercises[0].problem,
    };
  },
  mounted() {
    // Request match data from backend
  },
  methods: {
    handleSave,
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
  font-size: 1.5rem;
}

.profile p {
  margin: auto;
  padding-left: 10px;
}

.profilePic {
  height: 60px;
  width: 60px;
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

#editing,
#highlighting {
  /* Both elements need the same text and space styling so they are directly on top of each other */
  margin: 10px;
  padding: 10px;
  border: 0;
  width: calc(100% - 32px);
  height: 150px;
}

#editing,
#highlighting,
#highlighting * {
  /* Also add text styles to highlighting tokens */
  font-size: 15pt;
  font-family: monospace;
  line-height: 20pt;
}
</style>
