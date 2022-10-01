<template>
  <div class="content" id="app">
    <router-view></router-view>
  </div>
</template>

<script>
export default {
  name: 'App',
  components: {
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
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
  color: #FFFFFF;
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
