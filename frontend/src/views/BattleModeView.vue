<template>
  <main>
    <div class="infoBar">
      <div class="profile">
        <div class="profilePic"></div>
        <p>Player 1</p>
      </div>
      <div class="timer">
        <p>00:00.<span style="font-size:1.6rem">00</span></p>
      </div>
      <div class="profile">
        <div class="profilePic"></div>
        <p>Player 2</p>
      </div>
    </div>
    <div class="editorContent">
      <div class="editor"></div>
    </div>
    <div class="submit"><button class="button">submit</button></div>
  </main>
</template>

<script>
export default {

}
</script>

<style scoped>

main{
  display: block;
  width: 100%;
  height: 100%;
  margin: 0;
  margin-top: 30px;
}

.infoBar{
  display: flex;
  justify-content: space-between;
}

.profile{
  display: flex;
  font-size: 1.3rem;
}

.profile p{
  margin: auto;
  padding-left: 10px;
}

.profilePic {
  height: 40px;
  width: 40px;
  margin: auto;
  background-color: #bbb;
  border-radius: 50%;
  display: inline-block;
}

.timer{
  background: #e43717;
  display: inline-block;
  border-radius: 4px;
  padding: 10px 20px;
  width: 150px;
}

.timer p{
  text-align: center;
  color: white;
  font-size: 2.6rem;
  margin: 0;
  font-weight: bold;
}

.editorContent{
  height: 500px;
  margin: 0;
  padding: 60px 160px 60px 0;
}

.editor{
  height: 100%;
  border-radius: 4px;
  background: rgb(200, 200, 200);
  margin: 0;
}

.submit{
  text-align: end;
}

.button{
  width: 150px;
}
</style>