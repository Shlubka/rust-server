let selectedOption = "";

function toggleDropdown() {
  document.getElementById("dropdownContent").classList.toggle("show");
}

function selectOption(value, text) {
  selectedOption = value;
  document.getElementById("dropdownButton").textContent = text;
  document.getElementById("dropdownContent").classList.remove("show");
}

function showLanguageAlert(message) {
  alert(message);
  document.getElementById("dropdownContent").classList.remove("show");
}

window.onclick = function (event) {
  if (!event.target.matches("#dropdownButton")) {
    var dropdowns = document.getElementsByClassName("dropdown-content");
    for (var i = 0; i < dropdowns.length; i++) {
      var openDropdown = dropdowns[i];
      if (openDropdown.classList.contains("show")) {
        openDropdown.classList.remove("show");
      }
    }
  }
};

function preventFormSubmit(event) {
  event.preventDefault();
}

async function submitForm() {
  const textInput = document.getElementById("textInput").value.trim();

  if (textInput === "" || selectedOption === "") {
    document.getElementById("textInput").style.borderColor =
      textInput === "" ? "red" : "#ccc";
    document.getElementById("dropdownButton").style.borderColor =
      selectedOption === "" ? "red" : "#ccc";

    setTimeout(() => {
      document.getElementById("textInput").style.borderColor = "#ccc";
      document.getElementById("dropdownButton").style.borderColor = "#ccc";
    }, 2500);

    return;
  }

  const url = `http://127.0.0.1:8000/lang`;
  //const url = `https://buzzard-grateful-wren.ngrok-free.app/lang/`;

  const data = {
    language: selectedOption,
    code: textInput,
    promo: "admin",
  };

  try {
    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });

    if (response.redirected) {
      window.location.href = response.url;
    } else if (!response.ok) {
      throw new Error("Ошибка при отправке данных");
    }
  } catch (error) {
    alert("Произошла ошибка: " + error.message);
  }
}
