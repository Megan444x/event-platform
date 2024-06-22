import React from 'react';
import { render, fireEvent, waitFor, screen } from '@testing-library/react';
import '@testing-library/jest-dom/extend-expect';
import { rest } from 'msw';
import { setupServer as setupMockServer } from 'msw/node';
import EventDisplayComponent from './EventComponent';

const eventApiMockServer = setupMockServer(
  rest.get(process.env.REACT_APP_API_URL, (req, res, ctx) => {
    return res(ctx.json({ eventData: 'mocked event data' }));
  })
);

beforeAll(() => eventApiMockServer.listen());
afterEach(() => eventApiMockServer.resetHandlers());
afterAll(() => eventApiMockServer.close());

describe('EventDisplayComponent Tests', () => {
  it('renders the component as expected', async () => {
    render(<EventDisplayComponent />);
    expect(screen.getByTestId('event-component-root')).toBeInTheDocument();
  });

  it('dynamically updates display based on user input', async () => {
    render(<EventDisplayComponent />);
    fireEvent.change(screen.getByTestId('user-input-field'), { target: { value: 'user typed input' } });
    fireEvent.click(screen.getByTestId('update-data-button'));
    await waitFor(() => {
      expect(screen.getByTestId('dynamic-data-display')).toHaveTextContent('Updated data based on typed input');
    });
  });

  it('fetches and displays event data correctly', async () => {
    render(<EventDisplayComponent />);
    await waitFor(() => {
      expect(screen.getByTestId('fetched-data-display')).toHaveTextContent('mocked event data');
    });
  });

  it('responds accurately to user interactions', async () => {
    render(<EventDisplayComponent />);
    fireEvent.click(screen.getByTestId('main-action-button'));
    await waitFor(() => {
      expect(screen.getByTestId('action-outcome-display')).toHaveTextContent('Expected main action outcome');
    });
    fireEvent.mouseOver(screen.getByTestId('alternative-action-trigger'));
    await waitFor(() => {
      expect(screen.getByTestId('alternative-action-outcome')).toHaveTextContent('Expected alternative action outcome');
    });
  });
});