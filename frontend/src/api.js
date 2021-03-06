import axios from "axios";

const baseUrl = "/api/v1";

class API {
    static jwtGet(url) {
        let token = localStorage.getItem("token");
        if (!token) return new Promise((_, reject) => reject());
        return axios.get(url, { headers: { Authorization: `Token ${token}` }});
    }
    static jwtPost(url, data) {
        let token = localStorage.getItem("token");
        if (!token) return new Promise((_, reject) => reject());
        return axios.post(url, data, { headers: { Authorization: `Token ${token}` }});
    }

    static UserLogin(email, password) {
        return axios.post(baseUrl + "/user/login", {
            email,
            password,
        });
    }

    static UserRegister(email, username, password) {
        return axios.post(baseUrl + "/user/register", {
            email,
            username,
            password,
        });
    }

    static TeamCreate(name) {
        return API.jwtPost(baseUrl + "/team/create", {
            name,
        });
    }

    static TeamProfile(id) {
        if (id === undefined) {
            return API.jwtGet(baseUrl + "/team/me", {
                name,
            });
        } else {
            return API.jwtGet(baseUrl + "/team/profile/" + id, {
                name,
            });
        }
    }
}

export default API;
