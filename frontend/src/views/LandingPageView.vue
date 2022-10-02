<template>
  <main>
    <div class="content">
      <h1>
        READY FOR <br /><span style="font-size: 7.5rem; line-height: 50%"
          >BATTLE!</span
        >
      </h1>
      <img src="../assets/angryRust.png" alt="" />
    </div>
    <div class="buttonLayer">
      <button
        class="button"
        style="vertical-align: middle; font-size: 2rem; font-weight: bold"
        @click="openModal()"
      >
        start
      </button>
    </div>
    <div
      @click="modal = false"
      v-if="modal"
      style="width: 100vw; height: 100vh"
    >
      <div class="modalBackground" @click.stop>
        <div class="modal">
          <h2 style="margin-top: 80px; font-size: 2.4rem">
            Game is loading...
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
import router from "../router";

const findMatch = async () => {
  try {
    const response = await fetch("http://127.0.0.1:5000/match/find", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username: "Tomi",
      }),
    });
    const data = await response.json();
    console.log(data);
  } catch (error) {
    console.log(error);
  }
};

const joinMatch = async () => {
  try {
    const response = await fetch("http://127.0.0.1:5000/match/join", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username: "Tomi",
      }),
    });
    const data = await response.json();
    console.log(data);
  } catch (error) {
    console.log(error);
  }
};

export default {
  data: () => ({
    modal: false,
  }),
  mounted() {
    localStorage.setItem("exerciseId", 0);
  },
  methods: {
    async openModal() {
      this.modal = !this.modal;

      // Post request to find match
      await findMatch();

      setTimeout(async () => {
        // Post request to join match
        await joinMatch();
        router.push("/battle");
      }, 3000);
    },
  },
};
</script>

<style scoped>
main {
  width: 100%;
  height: 100%;
  margin: 0;
  margin-top: 100px;
}
.content {
  display: flex;
  justify-content: space-between;
  width: 100%;
  height: 100%;
  margin: 0;
}

h1 {
  font-size: 5rem;
  margin: 0;
}

.content img {
  width: 550px;
  height: fit-content;
  position: relative;
  top: 10px;
}

.buttonLayer {
  text-align: center;
  display: flex;
}

.buttonLayer .button {
  position: relative;
  left: 200px;
  bottom: 50px;
}

.buttonLayer .button {
  position: relative;
  left: 140px;
  bottom: 130px;
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
.modal img {
  width: 500px;
  position: relative;
  top: -75px;
  right: -120px;
}
</style>
