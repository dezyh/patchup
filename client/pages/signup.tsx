import { Stack, SlideFade, Flex, Heading, Input, Button, useColorModeValue, FormControl, FormErrorMessage } from '@chakra-ui/react'
import { Formik, Form, Field } from 'formik'
import {useDispatch} from 'react-redux'
import { signUpRequest } from '../store/user/actions'

const SignUp = () => {
  const dispatch = useDispatch()
  const formBackground = useColorModeValue('gray.200', 'gray.700')
  return (
    <SlideFade in offsetY='20px'>
      <Flex height='100vh' alignItems='center' justifyContent='center'>
        <Flex direction='column' background={formBackground} p={12} rounded={6} maxWidth='sm'>
          <Heading mb={6}>Sign Up</Heading>

          <Formik
            initialValues={{
              username: '',
              password: '',
              email: '',
              firstname: '',
              lastname: '',
            }}
            onSubmit={(values, actions) => {
              const { username, password, email, firstname, lastname } = values
              dispatch(signUpRequest(username, password, email, firstname, lastname))
            }}
          >
            
            <Form>

              <Field name='username'>
                {({ field, form }) => (
                  <FormControl isInvalid={form.errors.username && form.touched.username}>
                    <Input {...field} id='username' placeholder='Username' type='username' mb={3} variant='filled'/>
                    <FormErrorMessage>{form.errors.username}</FormErrorMessage>
                  </FormControl>
                )}
              </Field>

              <Field name='password'>
                {({ field, form }) => (
                  <FormControl isInvalid={form.errors.password && form.touched.password}>
                    <Input {...field} id='password' placeholder='Password' type='password' mb={3} variant='filled'/>
                    <FormErrorMessage>{form.errors.password}</FormErrorMessage>
                  </FormControl>
                )}
              </Field>

              <Field name='email'>
                {({ field, form }) => (
                  <FormControl isInvalid={form.errors.email && form.touched.email}>
                    <Input {...field} id='email' placeholder='Email Address' type='email' mb={3} variant='filled'/>
                    <FormErrorMessage>{form.errors.email}</FormErrorMessage>
                  </FormControl>
                )}
              </Field>

              <Stack direction='row'>

                <Field name='firstname'>
                  {({ field, form }) => (
                    <FormControl isInvalid={form.errors.firstname && form.touched.firstname}>
                      <Input {...field} id='firstname' placeholder='First Name' type='firstname' mb={3} variant='filled'/>
                      <FormErrorMessage>{form.errors.firstname}</FormErrorMessage>
                    </FormControl>
                  )}
                </Field>

                <Field name='lastname'>
                  {({ field, form }) => (
                    <FormControl isInvalid={form.errors.lastname && form.touched.lastname}>
                      <Input {...field} id='lastname' placeholder='Last Name' type='lastname' mb={3} variant='filled'/>
                      <FormErrorMessage>{form.errors.firstname}</FormErrorMessage>
                    </FormControl>
                  )}
                </Field>

              </Stack>

              <Button 
                colorScheme='blue'
                type='submit'
              >Sign Up
              </Button>

            </Form>

          </Formik>

        </Flex>
      </Flex>
    </SlideFade>
  )
}

export default SignUp
