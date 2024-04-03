import axios from "axios";
import { useState } from "react";
import { ScrollView, StyleSheet, View } from "react-native";
import { Avatar, Button, Card, Text, TextInput } from "react-native-paper";

const Login = () => {
    const [userName, setUserName] = useState("");
    const [password, setPassword] = useState("");

    const sendLoginRequest = () => {
        axios.post()
    }

    return (
        <>
            <View style={styles.container}>
                <ScrollView>
                    <Card>
                        <Card.Title title="Login"/>
                        <Card.Content>
                            <TextInput
                                mode="outlined"
                                label="Username"
                                right={<TextInput.Affix text="/100" />}
                                onChangeText={(value) => setUserName(value)}
                                value={userName}
                            />
                            <TextInput
                                mode="outlined"
                                label="Password"
                                right={<TextInput.Affix text="/100" />}
                                onChangeText={(value) => setPassword(value)}
                                value={password}
                            />
                        </Card.Content>
                        <Card.Actions>
                            <Button>Cancel</Button>
                            <Button>Ok</Button>
                        </Card.Actions>
                    </Card>                
                </ScrollView>
            </View>
        </>
    )
}   

const styles = StyleSheet.create({
    container: {
      overflow: "scroll",
    },
});

export default Login;