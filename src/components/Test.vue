<template>
	<div>
		<input type="file" accept="image/*" @change="handleFileChange" />
		Test
		<div>

		</div>
	</div>
</template>

<script setup>
import { ref } from 'vue';
const selectedImage = ref(null);

let firstImg
let filePath;
const handleFileChange = (e) => {
  filePath = e.target;
  const selectedFile = filePath.files[0];
  console.log(selectedFile)
  if (selectedFile) {
    firstImg=selectedFile

    const reader = new FileReader();
    reader.onload = () => {
      selectedImage.value = reader.result;

    };
  //  reader.readAsDataURL(selectedFile);
    reader.readAsBlob(selectedFile);

  }
  console.log(selectedImage);
};

</script>