import axios, { Method } from "axios";
import * as Device from 'expo-device';
import * as Keychain from 'react-native-keychain';


const BACKEND_URL = Device.isDevice ? 'http://localhost:8000/api/park-here' : 'http://10.0.2.2/api/park-here';

const httpMethodsMap : any = {
    'GET': axios.get,
    'PUT': axios.put,
    'POST': axios.post,
    'DELETE': axios.delete
}

const getHeaders = async (headers : any) => {
    try {
        const credentials = await Keychain.getGenericPassword();
        if(credentials) {
            return { ...headers, 'Authorization': credentials.password};
        } 
    } catch (error) {
        console.log(error)
        return {...headers};
    }
}

export const noBodyRequest = async (method:string, resource:string, customHeaders:object = {}) => {
    const headers = await getHeaders(customHeaders);

    try {
        let response = await httpMethodsMap[method](
            `${BACKEND_URL}/${resource}`,
            {
                headers
            }
        );
        return {'response': response, 'error': false};
    } catch (error) {
        console.log(error)
        return {'response': error, 'error': true} 
    }
}

export const request = async (method:string, resource:string, body:object, customHeaders:object = {}) => {
    const headers = await getHeaders(customHeaders);
    
    try {
        let response = undefined;
        
        response = await httpMethodsMap[method](
            `${BACKEND_URL}/${resource}`,
            body,
            {
                headers
            }
        )

        return {'response': response, 'error': false};
    } catch (error) {
        console.log(error)
        return {'response': error, 'error': true} 
    }
}