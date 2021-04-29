Vue.createApp({
  data: function () {
    return {
      // サムネイルリストの配列
      thumbnails: [
        {
          id: 1,
          src: "https://placehold.jp/300x300.png"
        },
        {
          id: 2,
          src: "https://placehold.jp/3d4070/ffffff/300x300.png"
        },
        {
          id: 3,
          src: "https://placehold.jp/b32020/ffffff/300x300.png"
        }
      ],
      selectedThumbnailId: undefined, //選択したサムネイルID
      isVisible: false, //trueなら表示、falseなら非表示
      thumbnailHeight: 0, //モーダル内のサムネイルの高さ
      isThumbnailLoaded: false, //サムネイルが読み込み完了したかどうか
    }
  },
  watch: {
    selectedThumbnailId: function() {
      this.isThumbnailLoaded = false
    }
  },
  computed: {
    //現在表示中のサムネイルオブジェクト
    currentThumbnail: function() {
      const self = this
      return _.find(self.thumbnails, function(thumb) {
        return thumb.id === self.selectedThumbnailId
      })
    },
    // 現在表示しているサムネイルの番号
    currentThumbnailIndex: function() {
      const self = this
      return _.findIndex(self.thumbnails, function(thumb) {
        return thumb.id === self.selectedThumbnailId
      })
    },
    nextThumbnail: function() {
      const nextIndex = this.currentThumbnailIndex + 1
      return this.thumbnails[
        nextIndex > this.thumbnails.length -1 ? 0 : nextIndex
      ]
    },
    prevThumbnail: function() {
      const prevIndex = this.currentThumbnailIndex - 1
      return this.thumbnails[
        prevIndex < 0 ? this.thumbnails.length -1 : prevIndex
      ]
    },
    containerStyle: function() {
      return {
        height: this.thumbnailHeight + "px"
      }
    }
  },
  methods: {
    openModal(thumb) {
      this.isVisible = true
      this.selectedThumbnailId = thumb.id
    },
    closeModal: function() {
      this.isVisible = false
      this.selectedThumbnailId = undefined
    },
    onClickPrev: function() {
      window.alert(this.prevThumbnail)
      this.selectedThumbnailId = this.prevThumbnail.id
    },
    onClickNext: function() {
      this.selectedThumbnailId = this.nextThumbnail.id
    },
    onLoad: function(event) {
      this.thumbnailHeight =
        event.target.naturalHeight > 300 ? 300 :
        event.target.naturalHeight
      this.isThumbnailLoaded = true
    }
  },
}).mount("#app") // Vueアプリケーションをマウントする要素

