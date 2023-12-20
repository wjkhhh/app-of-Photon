<script setup>
import { ref } from 'vue';


const selectedImage = ref(null);
const canvasRef = ref(null);
let firstImg
let filePath;
const handleFileChange = (e) => {
  filePath = e.target;
  const selectedFile = filePath.files[0];
  console.log(selectedFile)
  if (selectedFile) {
    firstImg=selectedFile
    // 使用 FileReader 读取文件并将其显示为图像
    const reader = new FileReader();
    reader.onload = () => {
      selectedImage.value = reader.result;

      const canvas = document.getElementById("myCanvas");
      const ctx = canvas.getContext('2d');
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      const img = new Image();
      img.onload = () => {
        canvas.width = img.width;
        canvas.height = img.height;
        ctx.drawImage(img, 0, 0);
      };
      img.src = reader.result;
    };
    reader.readAsDataURL(selectedFile);
  }
  console.log(selectedImage);
};
const threshold_p=ref(null);
const sepia_p=ref(null);
const alter_red_channel_p=ref(null);
const alter_blue_channel_p=ref(null);
const remove_blue_channel_p=ref(null);
const uploadPhoto=()=>{      
      console.log(filePath.files[0]);
      import("@silvia-odwyer/photon").then(photon => {
          var canvas = document.getElementById("myCanvas");
          var ctx = canvas.getContext("2d");
          let image = photon.open_image(canvas, ctx);
          if(threshold_p.value){
            photon.threshold(image,threshold_p.value);
          }
          if(sepia_p.value){
            photon.sepia(image);
          }
          if(alter_red_channel_p.value){
            photon.alter_red_channel(image,alter_red_channel_p.value);
          }
          if(alter_blue_channel_p.value){
            photon.alter_blue_channel(image,alter_blue_channel_p.value);
          }
          if(remove_blue_channel_p.value){
            photon.remove_blue_channel(image,remove_blue_channel_p.value);
          }
          photon.putImageData(canvas, ctx, image);
        console.log('ll')
      })
}
const firstPhoto=()=>{
  if (firstImg) {
    const reader = new FileReader();
    reader.onload = () => {
      selectedImage.value = reader.result;
      const canvas = document.getElementById("myCanvas");
      const ctx = canvas.getContext('2d');
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      const img = new Image();
      img.onload = () => {
        canvas.width = img.width;
        canvas.height = img.height;
        ctx.drawImage(img, 0, 0);
      };
      img.src = reader.result;
    };
    reader.readAsDataURL(firstImg);
  }
}

</script>

<template>
  <div>
    <input type="file" accept="image/*" @change="handleFileChange" />
    <button @click="uploadPhoto">修改</button>
    <button @click="firstPhoto">复原</button>

    <div class="select">

<div :title="`设置像素灰度值:0-255`">
<label for="threshold">threshold:</label>
<input type="text" id="threshold" v-model.number="threshold_p" :style="{width:'35px',marginRight:'10px'}">
</div>

<div :title="`图像转为棕色`" class="radio">
<label for="sepia">sepia:</label>
<input type="checkbox" id="sepia"  v-model="sepia_p" :style="{marginRight:'13px'}">
</div>
<div :title="`常量递增或递减每个像素的红色通道:-255到255`">
<label for="alter_red_channel">alter_red_channel:</label>
<input type="text" id="alter_red_channel" v-model.number="alter_red_channel_p" :style="{width:'35px',marginRight:'10px'}">
</div>
<div :title="`常量递增或递减每个像素的蓝色通道:-255到255`">
<label for="alter_blue_channel">alter_blue_channel:</label>
<input type="text" id="alter_blue_channel" v-model.number="alter_blue_channel_p" :style="{width:'35px',marginRight:'10px'}">
</div>
<div :title="`删除蓝色通道对图像的影响:-255到255`">
<label for="remove_blue_channel">remove_blue_channel:</label>
<input type="text" id="remove_blue_channel" v-model.number="remove_blue_channel_p" :style="{width:'35px',marginRight:'10px'}">
</div>

    </div>
    <div>
      <img :src="selectedImage" alt="Selected Image" v-if="selectedImage" :style="{width:'500px'}"/>
    </div>
  </div>
  <canvas id="myCanvas" ></canvas>
</template>
