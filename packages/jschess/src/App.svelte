<script lang="ts">
</script>

<main>
  <header>
    <h3>
      JS Chess 2.1
      <a href="http://www.bluefeversoft.com/Chess/versionHist.html" id="versionLink"
        >version history</a
      >
    </h3>
  </header>

  <div id="FenInDiv">
    <input type="text" id="fenIn" />
    <button type="button" id="SetFen">Set Position</button>
  </div>
  <div id="ThinkingImageDiv" />
  <div id="Board">
    <div class="Square clickElement rank8 file1 Dark" />
    <div class="Square clickElement rank8 file2 Light" />
    <div class="Square clickElement rank8 file3 Dark" />
    <div class="Square clickElement rank8 file4 Light" />
    <div class="Square clickElement rank8 file5 Dark" />
    <div class="Square clickElement rank8 file6 Light" />
    <div class="Square clickElement rank8 file7 Dark" />
    <div class="Square clickElement rank8 file8 Light" />
    <div class="Square clickElement rank7 file1 Light" />
    <div class="Square clickElement rank7 file2 Dark" />
    <div class="Square clickElement rank7 file3 Light" />
    <div class="Square clickElement rank7 file4 Dark" />
    <div class="Square clickElement rank7 file5 Light" />
    <div class="Square clickElement rank7 file6 Dark" />
    <div class="Square clickElement rank7 file7 Light" />
    <div class="Square clickElement rank7 file8 Dark" />
    <div class="Square clickElement rank6 file1 Dark" />
    <div class="Square clickElement rank6 file2 Light" />
    <div class="Square clickElement rank6 file3 Dark" />
    <div class="Square clickElement rank6 file4 Light" />
    <div class="Square clickElement rank6 file5 Dark" />
    <div class="Square clickElement rank6 file6 Light" />
    <div class="Square clickElement rank6 file7 Dark" />
    <div class="Square clickElement rank6 file8 Light" />
    <div class="Square clickElement rank5 file1 Light" />
    <div class="Square clickElement rank5 file2 Dark" />
    <div class="Square clickElement rank5 file3 Light" />
    <div class="Square clickElement rank5 file4 Dark" />
    <div class="Square clickElement rank5 file5 Light" />
    <div class="Square clickElement rank5 file6 Dark" />
    <div class="Square clickElement rank5 file7 Light" />
    <div class="Square clickElement rank5 file8 Dark" />
    <div class="Square clickElement rank4 file1 Dark" />
    <div class="Square clickElement rank4 file2 Light" />
    <div class="Square clickElement rank4 file3 Dark" />
    <div class="Square clickElement rank4 file4 Light" />
    <div class="Square clickElement rank4 file5 Dark" />
    <div class="Square clickElement rank4 file6 Light" />
    <div class="Square clickElement rank4 file7 Dark" />
    <div class="Square clickElement rank4 file8 Light" />
    <div class="Square clickElement rank3 file1 Light" />
    <div class="Square clickElement rank3 file2 Dark" />
    <div class="Square clickElement rank3 file3 Light" />
    <div class="Square clickElement rank3 file4 Dark" />
    <div class="Square clickElement rank3 file5 Light" />
    <div class="Square clickElement rank3 file6 Dark" />
    <div class="Square clickElement rank3 file7 Light" />
    <div class="Square clickElement rank3 file8 Dark" />
    <div class="Square clickElement rank2 file1 Dark" />
    <div class="Square clickElement rank2 file2 Light" />
    <div class="Square clickElement rank2 file3 Dark" />
    <div class="Square clickElement rank2 file4 Light" />
    <div class="Square clickElement rank2 file5 Dark" />
    <div class="Square clickElement rank2 file6 Light" />
    <div class="Square clickElement rank2 file7 Dark" />
    <div class="Square clickElement rank2 file8 Light" />
    <div class="Square clickElement rank1 file1 Light" />
    <div class="Square clickElement rank1 file2 Dark" />
    <div class="Square clickElement rank1 file3 Light" />
    <div class="Square clickElement rank1 file4 Dark" />
    <div class="Square clickElement rank1 file5 Light" />
    <div class="Square clickElement rank1 file6 Dark" />
    <div class="Square clickElement rank1 file7 Light" />
    <div class="Square clickElement rank1 file8 Dark" />
    <img src="./wR.png" class="Piece rank1 file1" /><img
      src="./wN.png"
      class="Piece rank1 file2"
    /><img src="./wB.png" class="Piece rank1 file3" /><img
      src="./wQ.png"
      class="Piece rank1 file4"
    /><img src="./wK.png" class="Piece rank1 file5" /><img
      src="./wB.png"
      class="Piece rank1 file6"
    /><img src="./wN.png" class="Piece rank1 file7" /><img
      src="./wR.png"
      class="Piece rank1 file8"
    /><img src="./wP.png" class="Piece rank2 file1" /><img
      src="./wP.png"
      class="Piece rank2 file2"
    /><img src="./wP.png" class="Piece rank2 file3" /><img
      src="./wP.png"
      class="Piece rank2 file4"
    /><img src="./wP.png" class="Piece rank2 file5" /><img
      src="./wP.png"
      class="Piece rank2 file6"
    /><img src="./wP.png" class="Piece rank2 file7" /><img
      src="./wP.png"
      class="Piece rank2 file8"
    /><img src="./bP.png" class="Piece rank7 file1" /><img
      src="./bP.png"
      class="Piece rank7 file2"
    /><img src="./bP.png" class="Piece rank7 file3" /><img
      src="./bP.png"
      class="Piece rank7 file4"
    /><img src="./bP.png" class="Piece rank7 file5" /><img
      src="./bP.png"
      class="Piece rank7 file6"
    /><img src="./bP.png" class="Piece rank7 file7" /><img
      src="./bP.png"
      class="Piece rank7 file8"
    /><img src="./bR.png" class="Piece rank8 file1" /><img
      src="./bN.png"
      class="Piece rank8 file2"
    /><img src="./bB.png" class="Piece rank8 file3" /><img
      src="./bQ.png"
      class="Piece rank8 file4"
    /><img src="./bK.png" class="Piece rank8 file5" /><img
      src="./bB.png"
      class="Piece rank8 file6"
    /><img src="./bN.png" class="Piece rank8 file7" /><img
      src="./bR.png"
      class="Piece rank8 file8"
    />
  </div>
  <div id="CurrentFenDiv">
    <span id="currentFenSpan">rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - KQkq 0 0</span>
  </div>
  <br />
  <div id="EngineOutput">
    Thinking Time: <br />
    <select id="ThinkTimeChoice">
      <option value="1">1s</option>
      <option value="2">2s</option>
      <option value="4">4s</option>
      <option value="6">6s</option>
      <option value="8">8s</option>
      <option value="10">10s</option></select
    ><br /><br /><br />

    <span id="BestOut">BestMove:</span><br />
    <span id="DepthOut">Depth:</span><br />
    <span id="ScoreOut">Score:</span><br />
    <span id="NodesOut">Nodes:</span><br />
    <span id="OrderingOut">Ordering:</span><br />
    <span id="TimeOut">Time:</span><br /><br />
    <button type="button" id="SearchButton">Move Now</button><br />
    <button type="button" id="NewGameButton">New Game</button><br />
    <button type="button" id="FlipButton">Flip Board</button><br /><br />
    <button type="button" id="TakeButton">Take Back</button><br /><br /><br />
    <span id="GameStatus" />
  </div>
</main>

<style lang="scss">
</style>
