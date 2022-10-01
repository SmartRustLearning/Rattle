<template>
  <main>
    <div class="content">
      <h1>READY FOR <br/>BATTLE</h1>
      <img src="../assets/angryRust.png" alt="">
    </div>
    <div class="buttonLayer">
      <router-link to ="/battle">
        <button class="button" style="vertical-align:middle">start</button>
      </router-link>
    </div>
    
  </main>
</template>

<script>
export default {

}
</script>

<style scoped>
main{
  width: 100%;
  height: 100%;
  margin: 0;
  margin-top: 100px;
}
.content{
  display: flex;
  justify-content: space-around;
  width: 100%;
  height: 100%;
  margin: 0;
}

h1{
  font-size: 5rem;
  margin: 0;
}

img{
  width: 400px;
  height: fit-content;
}

.buttonLayer{
  text-align: center;
}
</style>