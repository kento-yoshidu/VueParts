Vue.createApp({
  data: function() {
    return {
      items: [
        {
          id: 1,
          src: "https://placehold.jp/300x300.png"
        },
        {
          id: 2,
          src: "https://placehold.jp/3d4070/ffffff/300x200.png"
        },
        {
          id: 3,
          src: "https://placehold.jp/b32020/ffffff/300x400.png"
        }
      ],
      currentHeight: 0, // 現在表示しているカルーセル画像の高さ
      selectedIndex: 0, // 現在表示しているカルーセル画像のインデックス
      imageTransitionName: "prev" // トランジション名 prev OR next
    }
  },
  computed: {
    target: function() {
      const self = this
      return this.items[self.selectedIndex]
    },
    lastIndex: function() {
      return this.items.length - 1
    }
  },
  methods: {
    // ドットをクリックしたときのメソッド
    onClickPager: function(index) {
      console.log(this)
      this.imageTransitionName = this.selectedIndex < index ? "next" : "prev"
      this.selectedIndex = index
      this.setTargetHeight(index)
    },
    // PREVボタンををクリックしたときのメソッド
    onClickPrev: function () {
      this.imageTransitionName = "prev"
      // 最初の画像で「PREV」を押下した場合、最後の画像を表示する
      this.selectedIndex =
        this.selectedIndex <= 0 ? this.lastIndex : this.selectedIndex - 1
      this.setTargetHeight(this.selectedIndex)
    },
    // NEXTボタンををクリックしたときのメソッド
    onClickNext: function () {
      this.imageTransitionName = "next"
      // 最後の画像で「NEXT」を押下した場合、最初の画像を表示する
      this.selectedIndex =
        this.selectedIndex >= this.lastIndex ? 0 : this.selectedIndex + 1
      this.setTargetHeight(this.selectedIndex)
    },
    // 受け取ったインデックス番号の画像の高さを取得してimg要素の親に高さをセットするメソッド
    setTargetHeight: function (index) {
      const img = new Image()
      const self = this
      img.src = this.items[index].src
      img.onload = function (event) {
        // 画像が読み込まれたら画像の高さを親要素へセット
        self.currentHeight = event.currentTarget.height
      }
    }
  },
  mounted: function () {
    console.log(this.selectedIndex)
    this.setTargetHeight(this.selectedIndex)
  }
}).mount("#app")