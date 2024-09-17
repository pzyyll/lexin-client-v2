<script setup lang="ts">
enum Status {
  IDLE = "IDLE",
  LOADING = "LOADING",
  PLAYING = "PLAYING",
}

const status = defineModel<Status>({ default: Status.IDLE });

const emit = defineEmits<{
  onPlay: [
    {
      setLoading: () => void;
      play: (src: string) => void;
      isLoading: () => boolean;
      reset: () => void;
    }
  ];
  onPaused: [];
}>();

const audio = new Audio();
const onClick = () => {
  if (status.value === Status.IDLE) {
    emit("onPlay", { setLoading, play, isLoading, reset });
  } else {
    audio.pause();
    status.value = Status.IDLE;
    emit("onPaused")
  }
};

const play = (src: string) => {
  audio.pause();
  audio.src = src;
  audio.onplaying = () => {
    status.value = Status.PLAYING;
  };
  audio.onended = () => {
    status.value = Status.IDLE;
  };
  audio.onpause = () => {
    status.value = Status.IDLE;
  };
  audio.play();
};

const setLoading = () => {
  status.value = Status.LOADING;
};

const isLoading = () => {
  return status.value === Status.LOADING;
};

const reset = () => {
  status.value = Status.IDLE;
};

</script>

<template>
  <button class="relative" @click="onClick">
    <icon-gravity-ui-volume class="size-4" v-if="status == Status.IDLE" />
    <icon-ic-round-stop v-if="status == Status.LOADING || status == Status.PLAYING" />
    <span
      class="du-loading du-loading-spinner du-loading-sm absolute"
      v-if="status == Status.LOADING"
    ></span>
  </button>
</template>
