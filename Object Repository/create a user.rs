<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create a user</name>
   <tag></tag>
   <elementGuidId>b199c366-a010-4226-9a3b-9dd7e14bc7a8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;email&quot;,
      &quot;value&quot;: &quot;${email}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;openid&quot;,
      &quot;value&quot;: &quot;${openid}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;first_name&quot;,
      &quot;value&quot;: &quot;${first_name}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;last_name&quot;,
      &quot;value&quot;: &quot;${last_name}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;contact_no&quot;,
      &quot;value&quot;: &quot;${contact_no}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJraWQiOiJNVGhpTUdRMVlXTXRaVE5pTkMwME9XVXlMVGc0TmpVdE5HTm1ZVFkwWkRrNVpHVmoiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiJkbDgyNDJpNWdmbzdxM2FsOGIyOGpiNDVqIiwic3ViIjoiY2I3ODQxMDQtYTFlNy00Yzk2LTkwMmMtODI5N2RlZWMwN2IwIiwic2NvcGUiOiJlbWFpbCBvcGVuaWQgcHJvZmlsZSIsImtpZCI6Ik1UaGlNR1ExWVdNdFpUTmlOQzAwT1dVeUxUZzROalV0TkdObVlUWTBaRGs1WkdWaiIsImlzcyI6Imh0dHBzOlwvXC91c2Vycy52aXNlbnplLmNvbSIsImdyb3VwcyI6WyJtZXJjaGFudF9hZG1pbiJdLCJleHAiOjE1OTM3ODY0MjcsImlhdCI6MTU5Mzc0MzIyNywibm9uY2UiOiJXQWU1RGRBMGMzUDhVOE9jYndKSDh5YTBqM0g1QUVyMzQ4d3ZsZVhPIiwianRpIjoiNjVhZTU3MmMtNzBkMS00YTU3LWEyZWYtNmU5MjEyYjNhM2U5In0.F99tsMBnyUDvMc9rGsw0Mg94-510wwhv9ocxyqoN5rTGaWPF_y5H8XnobeeLeP0VsxH92acsSSPpbJo8_2X31AkqUE-qQr_UvpvEbyPF0jZDuPVRD6IpDxr8Q1SIB43bkAbhMq1NAEVRw-dBRdLa8Z_yapcF-0cW8g7hdVhVClq4hAQgIvGLnM5MZpef6617OwnZ3pElScxWQdfRpYWnvmrGhhoWtMkV61sONpYq4-cvaRxkSzmp_Hl6dGcQJuwgl91c_OAUYtFbpaVi8-VM20fRP6uXRANacWVMqMwo1UPH4aCW278q52RjxJF9iaIhqhF1G3tX9topzfhJYab2tg</value>
   </httpHeaderProperties>
   <katalonVersion>7.5.5</katalonVersion>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/api/users/signup</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bbdcf608-3ef2-4a8c-b477-1ba5143fafc1</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d546a388-24a1-452b-b17b-a1c509a09cb1</id>
      <masked>false</masked>
      <name>openid</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>783bacf1-9c9a-4205-b5ff-88197711a678</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5a41e050-56c1-49ec-8c76-de112ff1d013</id>
      <masked>false</masked>
      <name>last_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>62e4ebbc-3cb2-4706-9d78-2a4619aaa163</id>
      <masked>false</masked>
      <name>contact_no</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'status', &quot;OK&quot;)

WS.verifyElementPropertyValue(response, 'result.openid', &quot;0&quot;)

WS.verifyElementPropertyValue(response, 'result.email', &quot;test_0@gmail.com&quot;)

WS.verifyElementPropertyValue(response, 'result.first_name', &quot;test&quot;)

WS.verifyElementPropertyValue(response, 'result.last_name', &quot;0&quot;)

WS.verifyElementPropertyValue(response, 'result.contact_no', &quot;12345678&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
