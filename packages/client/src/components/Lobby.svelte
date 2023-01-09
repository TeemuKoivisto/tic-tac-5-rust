<script lang="ts">
  let name = 'unknown'

  function handlePlayClick() {}
</script>

<section
  class="my-32 p-4 h-full m-auto lg:container md:p-16 md:pt-8 xs:p-8 rounded-2xl flex flex-col items-center justify-center"
>
  <h1 class="my-3 text-5xl font-bold flex items-center">TicTac5</h1>
  <div class="mt-24 flex items-center justify-center">
    <div class="btn-container flex flex-col">
      <div class="col-field">
        <label for="name">Name</label>
        <input
          id="name"
          class="w-full py-2 px-2 bg-01 text-0B bg-zinc-600 rounded"
          bind:value={name}
        />
      </div>
      <div class="mt-10">
        <button
          class="btn focus:outline-none hover:bg-blue-700 hover:no-underline hover:-translate-y-1 hover:scale-110"
          on:click={() => handlePlayClick()}
        >
          Play against Human
        </button>
      </div>
    </div>
  </div>
</section>

<style lang="scss">
  .field {
    @apply flex items-center;
  }
  .field > label {
    @apply mr-2;
  }
  .field + .field {
    @apply mt-2;
  }
  .col-field {
    @apply flex flex-col;
  }
  .col-field + .col-field {
    @apply mt-2;
  }
  .col-field > label {
    @apply mb-2;
  }
  textarea {
    @apply p-2;
  }
</style>
