<template>
  <div class="content" id="app">
    <router-view></router-view>
  </div>
</template>

<script>
export default {
  name: "App",
  components: {},
  mounted() {
    // player
    localStorage.setItem("playerIdxRound1", 0);
    localStorage.setItem("playerIdxRound2", 1);
    localStorage.setItem("opponentIdxRound1", 2);
    localStorage.setItem("opponentIdxRound2", 3);

    // initial loading
    localStorage.setItem("loadingTimeInMillisRound1", 3000);
    localStorage.setItem("loadingTimeInMillisRound2", 7000);

    // waiting for results loading
    localStorage.setItem("computationTimeInMillisRound1", 6000);
    localStorage.setItem("computationTimeInMillisRound2", 2000);

    // exercises
    localStorage.setItem("exerciseId", 0);
    localStorage.setItem("isSecondGame", 0);

    // winner & loser
    localStorage.setItem("isWinnerOfGame1", 1);
    localStorage.setItem("isWinnerOfGame2", 0);
  },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: black;
  margin: 0;
  max-width: 1024px;
  height: 100%;
  margin: 0 auto;
  display: flex;
  /* background: bisque; */
}

.button {
  display: inline-block;
  border-radius: 4px;
  background-color: #e43717;
  border: none;
  color: #ffffff;
  text-align: center;
  font-size: 28px;
  padding: 20px;
  width: 200px;
  transition: all 0.5s;
  cursor: pointer;
  margin: 0;
}

.button:hover {
  transform: scale(1.05);
  transition: 200ms all;
}
</style>
