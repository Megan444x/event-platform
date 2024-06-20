import React from 'react';
import { render, fireEvent, waitFor, screen } from '@testing-library/react';
import '@testing-library/jest-dom/extend-expect';
import { rest } from 'msw';
import { setupMockServer } from 'msw/node';
import EventComponent from './EventComponent';

const mockServer = setupMockServer(
  rest.get(process.env.REACT_APP_API_ENDPOINT, (req, res, ctx) => {
    return res(ctx.json({ responseData: 'mocked data' }));
  })
);

beforeAll(() => mockServer.listen());
afterEach(() => mockServer.resetHandlers());
afterAll(() => mockServer.close());

describe('EventComponent Tests', () => {
  it('renders as expected', async () => {
    render(<EventComponent />);
    expect(screen.getByTestId('event-component-root')).toBeInTheDocument();
  });

  it('updates display in response to user input', async () => {
    render(<EventComponent />);
    fireEvent.change(screen.getByTestId('user-input-field'), { target: { value: 'new input' } });
    fireEvent.click(screen.getByTestId('data-update-button'));
    await waitFor(() => {
      expect(screen.getByTestId('live-data-display')).toHaveTextContent('Updated data based on new input');
    });
  });

  it('correctly fetches and shows data', async () => {
    render(<EventComponent />);
    await waitFor(() => {
      expect(screen.getByTestId('fetched-data-display')).toHaveTextContent('mocked data');
    });
  });

  it('accurately reacts to user actions', async () => {
    render(<EventComponent />);
    fireEvent.click(screen.getByTestId('primary-action-button'));
    await waitFor(() => {
      expect(screen.getByTestId('primary-action-response')).toHaveTextContent('Expected response to the action');
    });
    fireEvent.mouseOver(screen.getByTestId('secondary-action-trigger'));
    await waitFor(() => {
      expect(screen.getByTestId('secondary-action-response')).toHaveTextContent('Expected response to the other action');
    });
  });
});