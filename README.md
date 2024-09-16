# TripBuddy Web Service Project

This project aims to create an auto-balancing web server hosting a REST-based web service.  
The server is run on a virtual machine, which undergoes live migration to another physical machine as the increasing load (client requests to the web server) causes the CPU load to exceed a certain threshold.

Below are the instructions to run the web service called **TripBuddy**, which provides information on weather statistics as well as tourism hotspots of two input cities.

## Instructions to Compile, Test, and Run ProjectX TripBuddy Code

1. **Prerequisites:**  
   Make sure to install Maven 3.1 and Java 1.7, and update paths to point to `mvn` and `java`. Set Maven to use the IIITA proxy if required.  
   Set the `JAVA_HOME` and `M2_HOME` environment variables to point to your Java and Maven base folders.  
   Ensure `java`, `javac`, and `mvn` are in your execution path.

2. **Compile and Run TripBuddy REST Web Service on "host1":**  
   Run the following commands, passing the hostname of your machine as a parameter:

   ```bash
   cd cities/projectX
   mvn compile
   mvn exec:java -Dexec.mainClass="projectX.TripServiceLauncher" -Dexec.args="'host1' '8081'"
  
  You should be able to visit the web service URL:

  ```bash
  http://host1:8081/projectX/trip?msg1=London&msg2=Allahabad
  ```
from a browser (or using wget or curl commands) acting as a REST client. The response should be a JSON message.

3. **Compile and Run the Web Form Server on "host2":**
   Once the REST web service is running from step 2, compile and run the Web Form Server on "host2", passing the base URL location of the REST service and the public hostname of "host2" as parameters:
  
  ```bash
   cd cities/projectX
   mvn compile
   mvn exec:java -Dexec.mainClass="projectX.TripBuddyLauncher" -Dexec.args="'http://host1:8081/projectX/' 'host2' '8080'"
```
  You should be able to visit the web form on host2 using a browser. Submitting the form will trigger the REST client present in the form implementation and call the REST web service on host1.
```bash
http://host2:8080/projectX/web
```

## Live Migration Execution
The virt-exp master folder hosts the necessary files that monitor the CPU load and trigger live migration of the virtual machine to a desired physical host.




	
