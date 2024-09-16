document.querySelector("button").addEventListener("click", () =>{
    console.log("clicked");
    calculateTimezoneDifference();
});




function calculateTimezoneDifference() {
    var city1 = document.getElementById("city1").value;
    var city2 = document.getElementById("city2").value;
    getWeather(city1, city2);
    // getWeather(city2);
    var timezone1 = getTimezone(city1);
    var timezone2 = getTimezone(city2);

    if (timezone1 !== null && timezone2 !== null) {
        var difference = Math.abs(timezone1 - timezone2);
        document.getElementById("result1").innerHTML = "Timezone of " + city1 + ": " + timezone1;
        document.getElementById("result2").innerHTML = "Timezone of " + city2 + ": " + timezone2;
        document.getElementById("timezoneDifference").innerHTML = "Timezone Difference: " + difference + " hours";
    } else {
        alert("One or both of the cities you entered are not recognized. Please enter valid cities.");
    }
}

function getTimezone(city) {
    // Here you can implement a function or use an API to get the timezone of the city.
    // For simplicity, let's assume you have a mapping of cities to timezones.
    var timezoneMap = {
        "New York": -5,
        "Los Angeles": -8,
        "London": 0,
        "Paris": 1,
        // Add more cities and their respective timezones here
    };
    return timezoneMap[city];
}

const options = {
	method: 'GET',
	headers: {
		'X-RapidAPI-Key': 'c790acd260msh279a9f42061e4eap107834jsn3fec34e587d1',
		'X-RapidAPI-Host': 'weather-by-api-ninjas.p.rapidapi.com'
	}
};
        
async function getWeather(city1, city2) {
    const url1 = 'https://weather-by-api-ninjas.p.rapidapi.com/v1/weather?city=' + city1;
    const url2 = 'https://weather-by-api-ninjas.p.rapidapi.com/v1/weather?city=' + city2;

    // cityName1.innerHTML = city1;
    // cityName2.innerHTML = city2;
    // console.log(cityName1);

    try {
      const response1 = await fetch(url1, options);
      const result1 = await response1.json();
      console.log(result1);

    //   cloud_pct.innerHTML = result.cloud_pct;
    //   temp.innerHTML = result.temp;
    //   temp2.innerHTML = result.temp;
    //   humidity.innerHTML = result.humidity;
    //   humidity2.innerHTML = result.humidity;
      
      wind_speed1.innerHTML = "Wind Speed in " + city1 + " is: " +  result1.wind_speed;
      cloud_pct1.innerHTML = "Precipitation chances in " + city1 + " are : " + result1.cloud_pct + "%";
      min_temp1.innerHTML =  "Minimum Temperature of " + city1 + " is: " + result1.min_temp + "&#8451";
      max_temp1.innerHTML = "Maximum Temperature of " + city1 + " is: " +  result1.max_temp + "&#8451";
      humidity1.innerHTML = "Humidity in " + city1 + " is: " + result1.humidity + "%";

      const response2 = await fetch(url2, options);
      const result2 = await response2.json();
      console.log(result2);

      

      wind_speed2.innerHTML = "Wind Speed in " + city2 + " is: " +  result2.wind_speed;
      cloud_pct2.innerHTML = "Precipitation chances in " + city2 + " are : " + result2.cloud_pct + "%";
      min_temp2.innerHTML =  "Minimum Temperature of " + city2 + " is: " + result2.min_temp + "&#8451";
      max_temp2.innerHTML = "Maximum Temperature of " + city2 + " is: " +  result2.max_temp + "&#8451";
      humidity2.innerHTML = "Humidity in " + city2 + " is: " + result2.humidity + "%";
      // wind_speed2.innerHTML = result2.wind_speed;
      // cloud_pct2.innerHTML = result2.cloud_pct;
      // min_temp2.innerHTML = result2.min_temp;
      // max_temp2.innerHTML = result2.max_temp;
      // humidity2.innerHTML = result2.humidity;

    } catch (error) {
      console.error(error);
    }
    
  }
  
//   // Add an event listener to the submit button
//   submit.addEventListener('click', (e) => {
//     e.preventDefault();
//     // const city = cityName.value; // Get the value of the search box
//     // getWeather(city);
//     getWeather(city.value)
//   });
  
//   getWeather("Nagpur");

// const myModal = document.getElementById('myModal')
// const myInput = document.getElementById('myInput')

// myModal.addEventListener('click', () => {
//   myInput.focus()
// })