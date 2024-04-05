import axios from "axios";
import { useState } from "react";
import { ScrollView, StyleSheet, View } from "react-native";
import { Avatar, Button, Card, Dialog, Portal, Text, TextInput } from "react-native-paper";
import * as Keychain from 'react-native-keychain';

const Login = () => {
    const [userName, setUserName] = useState("");
    const [password, setPassword] = useState("");

    const [visible, setVisible] = useState(false);
    const hideDialog = () => setVisible(false);

    const sendLoginRequest = async () => {
        console.log("being called");
        try {
            const resp = await axios.post(
                'http://10.0.2.2:8000/api/park-here/login', 
                {id: userName, username: userName, passwd: password}
            );
            console.log(resp);
            Keychain.setGenericPassword(userName, resp.data)
        } catch (error) {
            console.log(error);
            setVisible(true);
        }
    };

    return (
        <View>
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
                            <Button onPress={() => sendLoginRequest()}>Login</Button>
                        </Card.Actions>
                    </Card>                
                </ScrollView>
            </View>
            <Portal>
                <Dialog visible={visible} onDismiss={hideDialog}>
                    <Dialog.ScrollArea>
                    <ScrollView contentContainerStyle={{paddingHorizontal: 24}}>
                        <Text>Authentication Error</Text>
                    </ScrollView>
                    </Dialog.ScrollArea>
                </Dialog>
            </Portal>
        </View>
    );
};

const styles = StyleSheet.create({
    container: {
      overflow: "scroll",
    },
});

export default Login;