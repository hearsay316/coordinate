<script setup>
async function handleChange(event) {
  const files = event.target.files;
  console.log(event, 'event');
  const formData = new FormData();
  for (let i = 0; i < files.length; i++) {
    formData.append('files[]', files[i]);
  }
  formData.append('text', "123");
  try {
    const response = await fetch('http://localhost:3303/app/api/users/12?ggg=hhh&hhh=123', {
      method: 'POST',
      body: formData,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }

    const result = await response.text();
    console.log('Success:', result);
  } catch (error) {
    console.error('Error:', error);
  }
}
</script>

<template>
  <main>
   <input type="file"  multiple @change="handleChange">
  </main>
</template>
